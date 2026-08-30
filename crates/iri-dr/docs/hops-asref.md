# `AsRef` Hops

The refchain pattern describes the transitive closure of the widening lattice concisely using hops: each hop's set is the direct target plus everything that target already reaches. This file derives the `AsRef` summary sets in [morphism.md](morphism.md).

## One Hop - Within Family

Each borrowed type `AsRef`s to every broader borrowed type in its family:

```
Iri           -AsRef→ { IriReference }
IriNormalized -AsRef→ { Iri, IriReference }
IriAbsolute   -AsRef→ { Iri, IriReference }
IriRelative   -AsRef→ { IriReference }

Uri           -AsRef→ { UriReference }
UriNormalized -AsRef→ { Uri, UriReference }
UriAbsolute   -AsRef→ { Uri, UriReference }
UriRelative   -AsRef→ { UriReference }
```

## Two Hop - Cross Family (URI → IRI)

Each URI borrowed type `AsRef`s to its IRI counterpart's set (itself + its one-hop set).

```
Uri           -AsRef→ { Iri           (hop adds: IriReference) }
UriNormalized -AsRef→ { IriNormalized (hop adds: Iri, IriReference) }
UriReference  -AsRef→ { IriReference }
UriAbsolute   -AsRef→ { IriAbsolute   (hop adds: Iri, IriReference) }
UriRelative   -AsRef→ { IriRelative   (hop adds: IriReference) }
```

## Three Hop - Cross Ownership (TypeBuf → Type)

Each `TypeBuf` `AsRef`s to its borrowed `Type`.

```
IriBuf           -AsRef→ { Iri           (hop adds: IriReference) }
IriNormalizedBuf -AsRef→ { IriNormalized (hop adds: Iri, IriReference) }
IriReferenceBuf  -AsRef→ { IriReference }
IriAbsoluteBuf   -AsRef→ { IriAbsolute   (hop adds: Iri, IriReference) }
IriRelativeBuf   -AsRef→ { IriRelative   (hop adds: IriReference) }

UriBuf           -AsRef→ { Uri           (hop adds: UriReference, Iri, IriReference) }
UriNormalizedBuf -AsRef→ { UriNormalized (hop adds: Uri, UriReference, IriNormalized, Iri, IriReference) }
UriReferenceBuf  -AsRef→ { UriReference  (hop adds: IriReference) }
UriAbsoluteBuf   -AsRef→ { UriAbsolute   (hop adds: Uri, UriReference, IriAbsolute, Iri, IriReference) }
UriRelativeBuf   -AsRef→ { UriRelative   (hop adds: UriReference, IriRelative, IriReference) }
```
