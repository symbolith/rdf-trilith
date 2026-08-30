# iri-dr overview

1. [morphism.md](morphism.md): trait-impl surface connecting the types
2. [validated.md](../../macro-validated-dr/docs/validated.md): trait-impl surface for borrowed/owned pairs and strings
3. [resolve.md](resolve.md): reference resolution and normalization

## Scope

`iri-dr` provides IRI (RFC 3987) and URI (RFC 3986) type families:

- four grammar productions: full, reference, absolute, relative
- one normalized form of the full production per family
- one borrowed/owned type pair each (10 pairs, 20 types)
- a single `Components<'a>` struct of component refs for O(1) access
- reference resolution per RFC 3986 §5 / RFC 3987 §6.5, specified in [resolve.md](resolve.md)

## Grammar

```
      IRI / URI   = scheme ":" hier-part [ "?" query ] [ "#" fragment ]

      hier-part   = "//" authority path-abempty
                  / path-absolute
                  / path-rootless
                  / path-empty
```

The following are two example URIs and their component parts:

```
        foo://example.com:8042/over/there?name=ferret#nose
        \_/   \______________/\_________/ \_________/ \__/
         |           |            |            |        |
      scheme     authority       path        query   fragment
         |   _____________________|__
        / \ /                        \
        urn:example:animal:ferret:nose
```

| Type               | RFC 3987 (IRI)   | RFC 3986 (URI)   | Scheme   | Fragment |
|--------------------|------------------|------------------|----------|----------|
| `IriReference`  | `IRI-reference`  | `URI-reference`  | optional | allowed  |
| `Iri`           | `IRI`            | `URI`            | required | allowed  |
| `IriNormalized` | normalized `IRI` | normalized `URI` | required | allowed  |
| `IriAbsolute`   | `absolute-IRI`   | `absolute-URI`   | required | no       |
| `IriRelative`   | `irelative-ref`  | `relative-ref`   | no       | allowed  |

## Types

### Type matrix

| Production  | IRI borrowed    | IRI owned          | URI borrowed    | URI owned          |
|-------------|-----------------|--------------------|-----------------|--------------------|
| full        | `Iri`           | `IriBuf`           | `Uri`           | `UriBuf`           |
| normalized  | `IriNormalized` | `IriNormalizedBuf` | `UriNormalized` | `UriNormalizedBuf` |
| reference   | `IriReference`  | `IriReferenceBuf`  | `UriReference`  | `UriReferenceBuf`  |
| absolute    | `IriAbsolute`   | `IriAbsoluteBuf`   | `UriAbsolute`   | `UriAbsoluteBuf`   |
| relative    | `IriRelative`   | `IriRelativeBuf`   | `UriRelative`   | `UriRelativeBuf`   |

`IriNormalized` / `UriNormalized`: `Iri` / `Uri` in [RFC 3986 §6.2.2](https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2) syntax-based normal form.

### Component newtypes

Each IRI/URI component gets its own newtype:

- `Scheme`
- `Authority`
- `Path`
- `Query`
- `Fragment`

No owned variants: components always borrow from a parent IRI/URI string.

### Invariants

- All types are `#[repr(transparent)]` newtypes over `str` (borrowed) or `String` (owned)
- All newtypes are generated via macro-validated-dr's `validated_str!` / `validated_string!` macros, so newtype boilerplate is never hand-written
- `Deref` is reserved for the owned→borrowed→`str` chain; subtype coercions use `AsRef` (matches the iri-string convention)
- The complete trait-impl surface connecting the types to each other is specified in [morphism.md](morphism.md); the borrowed/owned and string surface in [validated.md](../../macro-validated-dr/docs/validated.md)
