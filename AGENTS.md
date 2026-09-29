# AGENTS.md

Rust workspace of RDF building blocks. Every crate ships borrowed/owned type pairs (`Foo<'_>` / `FooBuf`) connected by widening morphisms.

## Crates

- `rdf-trilith-macro-validated`: `validated_str!` / `validated_string!` / `morphism!` macros generating validated `str` newtypes and their conversion surface
- `rdf-trilith-macro-enum`: `morphism!` macro generating enum widening impls and narrowing accessors
- `rdf-trilith-iri`: IRI type lattice (`Iri`, `IriAbsolute`, `IriNormalized`, ...) as `#[repr(transparent)]` `str` newtypes
- `rdf-trilith-term`: RDF 1.2 terms (`Iri`, `BlankNode`, `Literal`) and position enums (`Subject`, `Predicate`, `Object`, `Term`)
- `rdf-trilith-graph`: `Triple`/`TripleBuf` (named fields), `Graph`/`GraphMut` traits (GAT `type Triple<'g>`; `contains`/`remove` tie the triple lifetime to the `self` borrow), `MemoryGraph`
- `rdf-trilith-dataset`: `Quad`/`QuadBuf`, `GraphName`/`GraphNameBuf`, `Dataset`/`DatasetMut` (GATs mirroring rdf-trilith-graph; `graph()` returns `Option<&Self::Graph<'_>>` so generic code can drill into graphs), `MemoryDataset`; depends on rdf-trilith-graph
- `rdf-trilith-links`: markdown/wiki/autolink link forms

## Conventions

- Statements are flat named-field structs (oxrdf/rio style), never tuples; tuple `From` impls exist for terse construction
- Default graph is `Option::None`, never a sentinel variant
- Lifetime names are semantic: `'g` in rdf-trilith-graph, `'d` in rdf-trilith-dataset
- No generic statement trait and no `Source` trait; deliberate, do not reintroduce without the user
- oxrdf interop lives in per-crate `compat/oxrdf.rs` behind the `oxrdf` feature
- Design docs live in `crates/<crate>/docs/`; read them before touching a crate
- Verify with `cargo test --workspace --all-features` and `cargo clippy -- -W clippy::cognitive_complexity -W clippy::excessive_nesting` (thresholds in `clippy.toml`)
