# rdf-trilith-term overview

1. [morphism.md](morphism.md): conversion surface connecting term types and enums
2. [rdf-12.md](rdf-12.md): RDF 1.2 triple terms, the recursion problem, and literal structure

`rdf-trilith-term` provides the RDF 1.1 term types and the enums over them on top of [rdf-trilith-iri](../../rdf-trilith-iri/docs/overview.md). Where rdf-trilith-iri models a lattice of string grammars, rdf-trilith-term models sum types: a subject is an IRI *or* a blank node, so the types are enums, not `str` newtypes.

## Scope

- term types: `BlankNode`/`BlankNodeBuf`, `Literal` (IRIs come from rdf-trilith-iri)
- enums: `Subject`, `Object`, `Term`, each as a borrowed/owned pair
- `Predicate` is not an enum: it is exactly an IRI, aliased to `Iri`/`IriBuf`
- rdf-trilith-macro-enum's `morphism!` macro generating the conversion surface between term types and enums, and between enums

## Types

### Term types

`BlankNode`/`BlankNodeBuf` are `#[repr(transparent)]` newtypes generated with rdf-trilith-macro-validated's `validated_str!`/`validated_string!` macros. `Literal` is a component struct (lexical form, datatype, language tag) with no borrowed mirror; `LiteralBuf` is an alias for it, specified in [rdf-12.md](rdf-12.md).

| Term type  | Borrowed    | Owned          |
|------------|-------------|----------------|
| IRI        | `Iri`       | `IriBuf`       |
| blank node | `BlankNode` | `BlankNodeBuf` |
| literal    | `Literal`   | `LiteralBuf`   |

### Enums

| Slot      | Borrowed (`Copy`)                              | Owned                     |
|-----------|------------------------------------------------|---------------------------|
| subject   | `Subject<'a>`  = `Iri \| BlankNode`            | `SubjectBuf`              |
| predicate | `&'a Iri` (alias `Predicate<'a>`)              | `IriBuf` (`PredicateBuf`) |
| object    | `Object<'a>`   = `Iri \| BlankNode \| Literal` | `ObjectBuf`               |
| any       | `Term<'a>`     = `Iri \| BlankNode \| Literal` | `TermBuf`                 |

```rust
enum Subject<'a> { Iri(&'a Iri), BlankNode(&'a BlankNode) }
enum SubjectBuf  { Iri(IriBuf),  BlankNode(BlankNodeBuf) }
```

`Object` and `Term` share a variant set but stay distinct types: `Object` is the object slot of a triple, `Term` is a term in any slot. In RDF 1.2 both gain a `Triple` variant; see [rdf-12.md](rdf-12.md).
