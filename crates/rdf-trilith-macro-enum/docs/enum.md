# rdf-trilith-macro-enum

`morphism!` generates the conversion surface for enums whose variants hold validated newtypes: widening constructs a value (`From`), narrowing matches on the discriminant (`Option` accessors).

## morphism!

```
morphism!(Enum { Variant, ... })
morphism!(Narrow => Broad { SharedVariant, ... })
morphism!(Source as Variant => Enum)
```

Variant form, per variant plus the ownership bridge:

```
&'a Variant  -From→          { Enum<'a> }
VariantBuf   -From→          { EnumBuf }
Enum<'a>     -as_variant→    { Option<&'a Variant> }
EnumBuf      -into_variant→  { Option<VariantBuf> }

&'a EnumBuf  -From→          { Enum<'a> }
Enum<'a>     -From→          { EnumBuf }   (allocates)
```

Enum-to-enum form, widening by variant remapping:

```
Narrow<'a>   -From→          { Broad<'a> }
NarrowBuf    -From→          { BroadBuf }
Broad<'a>    -as_narrow→     { Option<Narrow<'a>> }
BroadBuf     -into_narrow→   { Option<NarrowBuf> }
```

Source form, widening a standalone type into a variant:

```
&'a Source   -From→          { Enum<'a> }
SourceBuf    -From→          { EnumBuf }
```
