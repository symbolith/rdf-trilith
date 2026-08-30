# IRI/URI Implementation Research in Rust

**Research Date:** 2026-01-29  
**Objective:** Design a universal IRI library for Rust

## Executive Summary

After analyzing 6 major IRI/URI implementations in Rust, the landscape shows distinct architectural approaches optimized for different use cases. The most successful libraries share these characteristics:

- **Zero-copy or minimal allocation** parsing strategies
- **Generic over storage types** (&str, String, Cow)
- **RFC 3986/3987 compliance** with varying degrees of strictness
- **Type-level safety** (separating absolute from relative IRIs)

---

## 1. Survey of Major Implementations

### Downloads & Popularity

| Crate | Downloads | Latest Version | Description |
|-------|-----------|----------------|-------------|
| **url** | 479M | 2.5.8 | WHATWG URL Standard (not RFC 3986/3987) |
| **iri-string** | 74M | 0.7.10 | IRI as string types with validation |
| **fluent-uri** | 28M | 0.4.1 | RFC 3986/3987 compliant URI/IRI handler |
| **uriparse** | 23M | 0.6.4 | URI parser with relative references |
| **iref** | 4.7M | 3.2.2 | URIs/IRIs borrowed and owned |
| **oxiri** | 482K | 0.2.11 | Simple, fast IRI validation & resolution |
| **rdftk_iri** | 34K | 0.2.5 | IRI/URI for RDF toolkit |

---

## 2. Detailed Architectural Analysis

### 2.1 IRI-STRING (74M downloads)

**Repository:** <https://github.com/lo48576/iri-string>

#### Core Architecture
- **String types as native slices**: Works like str/String with validation
- **Type-based safety**: Separate types for absolute vs relative, IRI vs URI
- **Zero-copy validation**: No allocation during parsing

#### Key Types
- Borrowed: &RiStr, &RiReferenceStr, &RiRelativeStr
- Owned: RiString, RiReferenceString, RiRelativeString
- Also has URI variants: UriStr, UriString, etc.

#### Design Philosophy
Native string semantics - types behave like str/String but with validation built in.

#### Strengths
✅ Most ergonomic API (feels like using native strings)  
✅ Type system prevents mixing absolute/relative  
✅ no_std support  
✅ IRI builder with validation  
✅ RFC 6570 URI Template support  
✅ Password masking  

#### Weaknesses
⚠️ Component access requires parsing on each call  
⚠️ No pre-computed offsets (trades memory for API simplicity)

---

### 2.2 FLUENT-URI (28M downloads)

**Repository:** <https://github.com/yescallop/fluent-uri-rs>

#### Core Architecture
- **Generic over URI/IRI**: Single implementation for both
- **EStr/EString**: Percent-encoded string types
- **Zero-copy component access**: Pre-computed during parse

#### Key Types
- Uri<T>, UriRef<T> - Generic over String/&str
- Iri<T>, IriRef<T> - IRI variants
- EStr<E> - Percent-encoded str with encoding context
- EString<E> - Percent-encoded String (growable)

#### Design Philosophy
Percent-encoding as first-class concept with specialized types and builder API.

#### Strengths
✅ Best-in-class percent encoding API  
✅ Split/decode percent-encoded components easily  
✅ Builder pattern for construction  
✅ Forbids unsafe code  
✅ Excellent documentation and examples  

#### Weaknesses
⚠️ More complex API surface (multiple encoding contexts)  
⚠️ Steeper learning curve

---

### 2.3 IREF (4.7M downloads)

**Repository:** <https://github.com/timothee-haudebourg/iref>

#### Core Architecture
- **Single-buffer mutable IRIs**: IriBuf holds IRI in one buffer, modifiable in-place
- **Borrowed and owned variants**: Iri<'a> (borrowed) vs IriBuf (owned)
- **Static IRI parsing**: Compile-time validation via static-iref crate

#### Key Types
- Iri<'a>, IriBuf - Borrowed and owned IRIs
- IriRef<'a>, IriRefBuf - IRI references
- Uri<'a>, UriBuf - URI variants

#### Design Philosophy
In-place mutation with path manipulation (push/pop segments) and normalization support.

#### Strengths
✅ Mutable IRIs (set_path, push_segment, etc.)  
✅ Compile-time validation with macros  
✅ In-place normalization  
✅ Comparison modulo normalization  

#### Weaknesses
⚠️ Single buffer can require more copies on modification  
⚠️ Less popular than alternatives

---

### 2.4 OXIRI (482K downloads) ⭐ MOST INNOVATIVE

**Repository:** <https://github.com/oxigraph/oxiri>

#### Core Architecture - Position-Based Zero-Copy

The most memory-efficient design in the ecosystem:

```rust
pub struct IriRef<T> {
    iri: T,                              // Original string
    positions: IriElementsPositions,     // Component boundaries
}

struct IriElementsPositions {
    scheme_end: usize,      // End of "scheme:"
    authority_end: usize,   // End of "//authority"
    path_end: usize,        // End of path
    query_end: usize,       // End of "?query"
    // Fragment implicitly from query_end to end
}

pub struct Iri<T>(IriRef<T>);  // Guaranteed absolute
```

Component access via string slicing:
```rust
pub fn scheme(&self) -> Option<&str> {
    if self.positions.scheme_end == 0 {
        None
    } else {
        Some(&self.iri[..self.positions.scheme_end - 1])
    }
}

pub fn path(&self) -> &str {
    &self.iri[self.positions.authority_end..self.positions.path_end]
}
```

#### Dual-Mode Parsing with Const Generics

```rust
struct IriParser<'a, O: OutputBuffer, const UNCHECKED: bool> {
    // State machine with optional validation
}
```

Two parsing modes:
- **UNCHECKED = false**: Full validation (default)
- **UNCHECKED = true**: Skip validation for trusted input

#### Output Buffer Abstraction

```rust
trait OutputBuffer {
    fn push(&mut self, c: char);
    fn push_str(&mut self, s: &str);
    // ...
}

// Two implementations:
impl OutputBuffer for String { ... }      // For actual output
impl OutputBuffer for VoidOutputBuffer { ... }  // For validation only (zero allocation!)
```

#### Resolution Algorithm

Inline resolution during parsing - resolves relative IRI while parsing, no second pass:

```rust
pub fn resolve(&self, iri: &str) -> Result<IriRef<String>, IriParseError> {
    let mut target_buffer = String::with_capacity(self.iri.len() + iri.len());
    let positions = IriParser::parse(iri, Some(self.as_ref()), &mut target_buffer)?;
    Ok(IriRef { iri: target_buffer, positions })
}
```

Also provides relativize() - compute relative IRI from two absolute IRIs (reverse operation).

#### Design Philosophy
Minimize allocations, maximize performance:
- Zero allocation for validation
- Single allocation for resolution
- Generic ownership (works with &str, String, Cow<str>)
- Type-safe absoluteness (Iri<T> vs IriRef<T>)

#### Strengths
✅ **Most memory-efficient**: Single allocation + 32 bytes of positions  
✅ **Fastest validation**: parse_unchecked() for trusted input  
✅ **Zero-copy validation**: No allocation for simple checks  
✅ **Inline resolution**: Resolve while parsing, single pass  
✅ **Generic storage**: IriRef<&str> vs IriRef<String>  
✅ **Forbids unsafe code**  

#### Weaknesses
⚠️ No normalization (case-preserving, scheme stored as "hTTp" not "http")  
⚠️ Immutable (can't modify components after construction)  
⚠️ Character-by-character parsing (not byte-oriented, slower for ASCII)

#### Memory Comparison
For "http://example.com/path?query=value#frag":
- **oxiri**: 1 allocation, ~88 bytes total
- **fluent-uri**: 1 allocation, ~88 bytes total
- **iri-string**: 1 allocation, ~56 bytes (but O(n) component access)
- **iref**: 1 allocation, ~120 bytes
- **uriparse**: 6+ allocations, ~240 bytes

---

### 2.5 URIPARSE (23M downloads)

**Repository:** <https://github.com/sgodwincs/uriparse-rs>

#### Core Architecture
Traditional component-based parsing with separate fields for each part.

#### Strengths
✅ Simple, traditional API  
✅ Good RFC 3986 compliance  

#### Weaknesses
⚠️ More allocations than zero-copy alternatives  
⚠️ Last updated 2022 (less active maintenance)

---

### 2.6 RDFTK_IRI (34K downloads)

**Repository:** <https://github.com/johnstonskj/rust-rdftk>

Part of larger RDF toolkit, tailored for semantic web use cases.

#### Strengths
✅ Integrated RDF ecosystem  
✅ Semantic web focus  

#### Weaknesses
⚠️ Less general-purpose  
⚠️ Smaller community

---

## 3. Design Pattern Comparison

| Pattern | iri-string | fluent-uri | iref | oxiri | uriparse |
|---------|------------|------------|------|-------|----------|
| **Zero-copy validation** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Position-based** | ❌ | ✅ | ❌ | ✅✅ | ❌ |
| **Component pre-parsing** | ❌ | ✅ | ❌ | ✅ | ✅ |
| **Mutable** | ❌ | ❌ | ✅ | ❌ | ✅ |
| **Normalization** | ✅ | ✅ | ✅ | ❌ | ✅ |
| **no_std** | ✅ | ✅ | ✅ | ❌ | ❓ |
| **Const validation** | ❌ | ❌ | ✅ | ✅ | ❌ |
| **Generic storage** | ❌ | ✅ | ❌ | ✅✅ | ❌ |

---

## 4. Key Architectural Decisions

### 4.1 String Storage Strategies

#### A. Native String Types (iri-string)
Types act like str/String with built-in validation.

**Pros:** Most ergonomic, familiar API  
**Cons:** Component access requires runtime parsing

#### B. Position-Based (oxiri, fluent-uri) ⭐
Store original string + byte offsets for component boundaries.

**Pros:** O(1) component access, minimal memory  
**Cons:** No normalization, immutable

#### C. Component-Based (uriparse, url)
Parse into separate String fields for each component.

**Pros:** Easy mutation, normalized  
**Cons:** Multiple allocations, more memory

#### D. Single-Buffer Mutable (iref)
All components in one buffer with tracked offsets.

**Pros:** In-place mutation, single allocation  
**Cons:** Modification can require shifting data

---

### 4.2 Type Safety Patterns

#### Separate Types (iri-string, fluent-uri, oxiri) ⭐
Use distinct types for absolute vs relative IRIs.

Compiler prevents mixing absolute/relative at type level.

#### Runtime Check (iref, uriparse)
Single type with is_absolute() method.

Simpler API, more flexible, but less type-safe.

---

### 4.3 Parsing Strategies

#### Character-by-Character State Machine (oxiri, iri-string)
Simple logic, good for Unicode handling.

#### Byte-Scanning with SIMD (url crate)
Very fast for ASCII content using SIMD instructions.

---

## 5. Standards Compliance

| Crate | RFC 3986 (URI) | RFC 3987 (IRI) | WHATWG URL |
|-------|----------------|----------------|------------|
| iri-string | ✅ | ✅ | ❌ |
| fluent-uri | ✅ | ✅ | ❌ |
| iref | ✅ | ✅ | ❌ |
| oxiri | ✅ | ✅ | ❌ |
| uriparse | ✅ | ❌ | ❌ |
| url | ❌ | ❌ | ✅ |

**Key Differences:**
- **URI (RFC 3986):** ASCII-only, percent-encode non-ASCII
- **IRI (RFC 3987):** Allows Unicode directly (é, 中文, etc.)
- **WHATWG URL:** Browser-focused, looser parsing for compatibility

---

## 6. Recommendations for Universal IRI Library

### Core Design Principles

1. **Hybrid Architecture**
   - Zero-copy validation (like oxiri)
   - Position-based storage (like oxiri/fluent-uri)
   - Optional component pre-parsing (like fluent-uri)

2. **Type Safety**
   - Separate Iri<T> (absolute) from IriRef<T> (relative)
   - Compile-time validation macros (like iref)

3. **Generic Storage**
   - Work with &str, String, Cow<str>, Arc<str>
   - Zero-allocation for borrowed types

4. **Dual-Mode Parsing**
   - Validated (default): Full RFC 3987 compliance
   - Unchecked (opt-in): For trusted input

### Proposed API Surface

```rust
// Core types
pub struct Iri<T: Deref<Target = str>> {
    data: T,
    positions: Positions,  // Pre-computed on parse
}

pub struct IriRef<T: Deref<Target = str>> {
    data: T,
    positions: Positions,
}

// Compile-time validation
const BASE: Iri<&'static str> = iri!("http://example.com");

// Zero-allocation validation
let valid = Iri::validate("http://example.com")?;

// Parsing
let iri = Iri::parse("http://example.com")?;
let iri = Iri::parse_unchecked("http://...");

// Component access (O(1))
let scheme = iri.scheme();
let authority = iri.authority();
let path = iri.path();

// Resolution
let base = Iri::parse("http://example.com/base")?;
let relative = IriRef::parse("../other")?;
let resolved = base.resolve(&relative)?;

// Builder
let iri = IriBuilder::new()
    .scheme("https")
    .authority("example.com")
    .path("/api/v1")
    .query("key=value")
    .build()?;

// Mutable construction
let mut iri = IriMut::from(base);
iri.set_path("/new/path");
iri.push_query_param("key", "value");
let iri: Iri<String> = iri.freeze();
```

### Feature Flags

```toml
[features]
default = ["std", "alloc"]
std = ["alloc"]
alloc = []
no_std = []

# Extensions
serde = ["dep:serde"]
macros = []
builder = []
mutable = ["alloc"]
normalization = []

# Performance
simd = ["dep:memchr"]
unchecked = []
```

### Key Innovations to Include

1. **From oxiri:**
   - Position-based zero-copy architecture
   - Dual-mode parsing (validated/unchecked)
   - Inline resolution algorithm
   - Generic storage (IriRef<&str> vs IriRef<String>)

2. **From fluent-uri:**
   - Percent-encoding types (EStr, EString)
   - Builder API
   - Component iteration

3. **From iref:**
   - Compile-time validation macros
   - In-place mutation (via IriMut type)
   - Path segment manipulation

4. **From iri-string:**
   - Native string-like API
   - no_std support
   - URI Template (RFC 6570) support

---

## 7. Use Case Recommendations

| Use Case | Recommended Library | Reason |
|----------|---------------------|--------|
| **General web dev** | fluent-uri | Balance of features/performance |
| **RDF/Semantic Web** | iri-string or iref | IRI-first, no_std |
| **High-performance validation** | oxiri | Zero-allocation validation |
| **Mutable URI construction** | iref or uriparse | In-place modification |
| **Browser/WHATWG** | url | WHATWG compliance |
| **Embedded systems** | iri-string | no_std support |
| **Compile-time validation** | iref | Static iri! macro |

---

## 8. Conclusion

The ideal **universal IRI library** should combine:

1. **oxiri's memory efficiency** (position-based, zero-copy)
2. **fluent-uri's ergonomics** (percent-encoding types, builder)
3. **iri-string's breadth** (templates, normalization, no_std)
4. **iref's compile-time safety** (static macros, mutation)

This hybrid approach provides:
- ✅ Minimal allocations for validation
- ✅ O(1) component access
- ✅ Type-safe absolute/relative distinction
- ✅ Compile-time and runtime validation
- ✅ Both immutable and mutable APIs
- ✅ no_std support
- ✅ RFC 3986/3987 compliance
- ✅ Excellent ergonomics

**Next Steps:**
1. Prototype core Iri<T>/IriRef<T> types with position storage
2. Implement state machine parser with dual-mode validation
3. Add builder and mutation APIs
4. Benchmark against existing implementations
5. Add percent-encoding utilities
6. Implement RFC 6570 URI Templates
7. Create comprehensive test suite

---

## References

- [RFC 3986 - URI](https://www.rfc-editor.org/rfc/rfc3986.html)
- [RFC 3987 - IRI](https://www.rfc-editor.org/rfc/rfc3987.html)
- [RFC 6570 - URI Template](https://www.rfc-editor.org/rfc/rfc6570.html)
- [WHATWG URL Standard](https://url.spec.whatwg.org/)

**Generated:** 2026-01-29

---

## Appendix: Complete Type Signature Analysis

For detailed analysis of type signatures across all implementations, see:

📄 **[Complete Type Signature Analysis](./iri-type-signatures.md)**

This document provides:
- Full type definitions for all major crates
- Memory layout comparisons
- Generic parameter patterns
- Validation strategy differences
- Recommended type system design

