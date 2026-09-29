# `From`/`Into` Hops

The refchain pattern describes the transitive closure of the widening lattice concisely using hops: each hop's set is the direct target plus everything that target already reaches. This file derives the `From`/`Into` summary sets in [morphism.md](morphism.md).

## One Hop - Within Owned Family

Each narrower `TypeBuf` converts into every broader `TypeBuf` in its family:

```
IriBuf           -Into→ { IriReferenceBuf }
IriNormalizedBuf -Into→ { IriBuf, IriReferenceBuf }
IriAbsoluteBuf   -Into→ { IriBuf, IriReferenceBuf }
IriRelativeBuf   -Into→ { IriReferenceBuf }

UriBuf           -Into→ { UriReferenceBuf }
UriNormalizedBuf -Into→ { UriBuf, UriReferenceBuf }
UriAbsoluteBuf   -Into→ { UriBuf, UriReferenceBuf }
UriRelativeBuf   -Into→ { UriReferenceBuf }
```

## Two Hop - Cross Owned Family (URI → IRI)

Each URI owned type converts `Into` its IRI counterpart's set (itself + its one-hop set):

```
UriBuf           -Into→ { IriBuf           (hop adds: IriReferenceBuf) }
UriNormalizedBuf -Into→ { IriNormalizedBuf (hop adds: IriBuf, IriReferenceBuf) }
UriReferenceBuf  -Into→ { IriReferenceBuf }
UriAbsoluteBuf   -Into→ { IriAbsoluteBuf   (hop adds: IriBuf, IriReferenceBuf) }
UriRelativeBuf   -Into→ { IriRelativeBuf   (hop adds: IriReferenceBuf) }
```
