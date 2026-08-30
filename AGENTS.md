# AGENTS.md

Rust workspace of RDF building blocks. Every crate ships borrowed/owned type pairs (`Foo<'_>` / `FooBuf`) connected by widening morphisms.

## Crates

- `macro-validated-dr`: `validated_str!` / `validated_string!` / `morphism!` macros generating validated `str` newtypes and their conversion surface
- `macro-enum-dr`: `morphism!` macro generating enum widening impls and narrowing accessors
- `iri-dr`: IRI type lattice (`Iri`, `IriAbsolute`, `IriNormalized`, ...) as `#[repr(transparent)]` `str` newtypes
- `term-dr`: RDF 1.2 terms (`Iri`, `BlankNode`, `Literal`) and position enums (`Subject`, `Predicate`, `Object`, `Term`)
- `graph-dr`: `Triple`/`TripleBuf` (named fields), `Graph`/`GraphMut` traits (GAT `type Triple<'g>`; `contains`/`remove` tie the triple lifetime to the `self` borrow), `MemoryGraph`
- `dataset-dr`: `Quad`/`QuadBuf`, `GraphName`/`GraphNameBuf`, `Dataset`/`DatasetMut` (GATs mirroring graph-dr; `graph()` returns `Option<&Self::Graph<'_>>` so generic code can drill into graphs), `MemoryDataset`; depends on graph-dr
- `links-dr`: markdown/wiki/autolink link forms

## Conventions

- Statements are flat named-field structs (oxrdf/rio style), never tuples; tuple `From` impls exist for terse construction
- Default graph is `Option::None`, never a sentinel variant
- Lifetime names are semantic: `'g` in graph-dr, `'d` in dataset-dr
- No generic statement trait and no `Source` trait; deliberate, do not reintroduce without the user
- oxrdf interop lives in per-crate `compat/oxrdf.rs` behind the `oxrdf` feature
- Design docs live in `crates/<crate>/docs/`; read them before touching a crate
- Verify with `cargo test --workspace --all-features` and `cargo clippy -- -W clippy::cognitive_complexity -W clippy::excessive_nesting` (thresholds in `clippy.toml`)
