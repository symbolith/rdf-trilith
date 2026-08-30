# Reference Resolution

`resolve` implements RFC 3986 §5 reference resolution.

## Signatures


```rust
impl IriReference { pub fn resolve(&self, base: impl AsRef<IriAbsolute>) -> IriBuf }
impl IriRelative  { pub fn resolve(&self, base: impl AsRef<IriAbsolute>) -> IriBuf }
impl IriAbsolute  { pub fn resolve(&self, reference: impl AsRef<IriReference>) -> IriBuf }

impl UriReference { pub fn resolve(&self, base: impl AsRef<UriAbsolute>) -> UriBuf }
impl UriRelative  { pub fn resolve(&self, base: impl AsRef<UriAbsolute>) -> UriBuf }
impl UriAbsolute  { pub fn resolve(&self, reference: impl AsRef<UriReference>) -> UriBuf }
```

## Semantics

- Strict mode (RFC 3986 §5.2.2): a reference with a scheme wins outright, even when the scheme equals the base's; `"http:g"` resolves to `"http:g"`.
- Merge (§5.2.3) and `remove_dot_segments` (§5.2.4) apply to relative paths; the output never contains complete `.` or `..` segments.
- Recomposition (§5.3) with one guard the RFC omits: an authority-less result whose path starts with `//` is serialized with a `/.` path prefix (WHATWG URL serialization), because `urn://x` would otherwise reparse with authority `x`.
