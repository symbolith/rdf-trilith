# RDF 1.2

RDF 1.2 ([rdf12-concepts](https://www.w3.org/TR/rdf12-concepts/)) extends the 1.1 term model in two ways that affect rdf-trilith-term:

- triple terms: a triple may appear as a term, but **only in the object slot**.
- literals gain an optional base direction (`dir`), alongside language tag and datatype.

## Triples

A triple is a plain tuple, no struct:

```rust
type Predicate<'a> = &'a Iri;
type PredicateBuf  = IriBuf;
type Triple<'a>    = (Subject<'a>, Predicate<'a>, Object<'a>);
type TripleBuf     = (SubjectBuf, PredicateBuf, ObjectBuf);
```

## The recursion problem

`Object` now contains a triple, and a triple contains an `Object`. This self-reference breaks both sides of the borrowed/owned split naively written.

Owned side: infinite size.

```rust
enum ObjectBuf {
    // ...
    Triple(TripleBuf),    // error[E0072]: recursive type has infinite size
}
```

`size_of::<ObjectBuf>()` would depend on itself. The fix is indirection: `Box<TripleBuf>` stores a fixed-size owning pointer to a heap value.

Borrowed side: no value to reference. A borrowed variant `Object<'a>::Triple(&'a Triple<'a>)` would need a `Triple<'a>` (a tuple of borrowed terms) to already exist in memory. A graph stores only `TripleBuf` values, so the owned→borrowed conversion would have to construct the borrowed tuple as a local and return a reference to it, which dangles (error[E0515]).

## Prior art

- oxrdf: owned `Term::Triple(Box<Triple>)` (feature `rdf-12`); borrowed `TermRef<'a>::Triple(&'a Triple)` references the **owned** triple. The borrowed enums stay `Copy`.
- sophia: single `SimpleTerm<'a>` enum over maybe-owned strings (`MownStr`); `Triple(Box<[SimpleTerm; 3]>)` owns its inner terms. No borrowed/owned split, not `Copy`.
- rdf-types: generic `Term<I, L>`; representation and recursion pushed to the instantiator.

## Design

rdf-trilith-term follows oxrdf: the borrowed triple variant references the owned tuple.

```rust
enum ObjectBuf {
    Iri(IriBuf),
    BlankNode(BlankNodeBuf),
    Literal(LiteralBuf),
    Triple(Box<TripleBuf>),
}

enum Object<'a> {
    Iri(&'a Iri),
    BlankNode(&'a BlankNode),
    Literal(&'a Literal),
    Triple(&'a TripleBuf),
}

impl<'a> From<&'a ObjectBuf> for Object<'a> {
    fn from(value: &'a ObjectBuf) -> Self {
        match value {
            ObjectBuf::Iri(iri) => Object::Iri(iri),
            ObjectBuf::BlankNode(blank_node) => Object::BlankNode(blank_node),
            ObjectBuf::Literal(literal) => Object::Literal(literal),
            ObjectBuf::Triple(triple) => Object::Triple(triple),
        }
    }
}
```

## Variant Sets in 1.2

| Slot      | Variant set                              |
|-----------|------------------------------------------|
| subject   | `Iri \| BlankNode`                       |
| predicate | `Iri`                                    |
| object    | `Iri \| BlankNode \| Literal \| Triple`  |
| any term  | `Iri \| BlankNode \| Literal \| Triple`  |

`Object` and `Term` still share a variant set in 1.2; they remain distinct types because they answer different questions (the object slot of a triple vs a term in any slot).

## Literals

A 1.2 literal has four components: lexical form, datatype IRI, optional language tag, optional base direction. Term equality is component-wise, and the components exist since 1.1 (direction is the only 1.2 addition), so the literal is a struct, not a validated `str` over an N-Triples lexical form:

```rust
struct Literal {
    lexical_form: String,
    datatype: IriBuf,
    language: Option<LanguageTagBuf>,
    direction: Option<Direction>,
}
```

There is no borrowed mirror struct: like the `Triple` variant, the borrowed enums reference the owned struct as `&'a Literal` (`LiteralBuf` is an alias). Quoting and escaping belong to serialization, not the data model.
