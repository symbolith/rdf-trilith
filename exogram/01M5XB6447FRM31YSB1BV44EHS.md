---
aliases:
  - Software RDF Trilith Interpretation
type:
  - "[Type Software](20260330120001.md)"
owner:
  - "[Symbolith](20260816134144.md)"
repository:
  - "[Repository symbolith/rdf-trilith](01M614DPSA1V80VDM3HNT2S057.md)"
comment: an RDF interpretation over RDF Fusion in which an IRI denotes a Rust value
wasInfluencedBy:
  - "[Software RDF Fusion](01M5XB63JSWPQ7KDAR8G8QN075.md)"
  - "[Software linked-data-rs](20250320132259.md)"
  - "[Software linked-data-sparql](01M5XB63W1AZTT0RZMVPB53W19.md)"
vetting: unread
---

The `Store` of [Software RDF Fusion](01M5XB63JSWPQ7KDAR8G8QN075.md) is async.
[Software linked-data-rs](20250320132259.md) is sync.
The crate bridges the two.

## Write

[Software linked-data-rs](20250320132259.md) serializes a Rust value into quads.
[Software RDF Fusion](01M5XB63JSWPQ7KDAR8G8QN075.md) stores them.

## Read

Asked for the interpretation of an IRI as a Rust type, the crate takes the type's RDF annotations.
[Software linked-data-sparql](01M5XB63W1AZTT0RZMVPB53W19.md) builds the query from them.
[Software RDF Fusion](01M5XB63JSWPQ7KDAR8G8QN075.md) runs it.
[Software linked-data-rs](20250320132259.md) deserializes the result into the Rust value.
