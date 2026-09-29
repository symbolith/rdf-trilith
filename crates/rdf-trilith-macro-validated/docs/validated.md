# rdf-trilith-macro-validated

Each macro defines a `#[repr(transparent)]` validated newtype and its trait surface. `Deref` is reserved for the owned→borrowed→str chain. `Borrow` guarantees identical `Eq`/`Ord`/`Hash` results for borrowed and owned forms, enabling collection key lookups by reference. `Display` gives every generated type `ToString` via std's blanket impl.

## validated_str!

```
validated_str!(vis Type, validate => Error)
```

```
Type     -Deref→    { str }
Type     -AsRef→    { Type, str }
Type     -Borrow→   { str }
&str     -TryFrom→  { Result<&Type, E> }     (validates, no alloc)
Type     -Display→  { fmt::Result }

(+ Debug, Eq, PartialEq, Hash, Ord, PartialOrd)
```

## validated_string!

```
validated_string!(vis TypeBuf for Type, validate => Error)
```

```
TypeBuf  -Deref→    { Type }
TypeBuf  -AsRef→    { Type, str }
TypeBuf  -Borrow→   { Type, str }
Type     -ToOwned→  { TypeBuf }              (allocates)
&Type    -From→     { TypeBuf }              (allocates)
&str     -FromStr→  { Result<TypeBuf, E> }   (validates + allocates)
String   -TryFrom→  { Result<TypeBuf, E> }   (validates, no realloc)
TypeBuf  -Into→     { String }               (unwraps, no alloc)
TypeBuf  -Display→  { fmt::Result }

(+ Clone, Debug, Eq, PartialEq (also vs Type, &Type), Hash, Ord, PartialOrd)
```

## morphism!

```
morphism!(NarrowBuf for Narrow => BroadBuf for Broad)
```

The invoker asserts that every string accepted by the narrow validator is also accepted by the broad one; the widening impls rely on it without re-validating.

```
Narrow     -AsRef→   { Broad }                  (pointer cast)
NarrowBuf  -AsRef→   { Broad }                  (pointer cast)
NarrowBuf  -Into→    { BroadBuf }               (unwraps, no realloc)
&Broad     -TryFrom→ { Result<&Narrow, E> }     (validates, no alloc)
BroadBuf   -TryFrom→ { Result<NarrowBuf, E> }   (validates, no realloc)
```
