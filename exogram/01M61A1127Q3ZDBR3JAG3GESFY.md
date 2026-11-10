---
aliases:
  - Add an interpretation layer to RDF Trilith
type:
  - "[Type Act](20260608023505.md)"
hasState: intended
isPartOf:
  - "[Project Rebase RDF Trilith on rdf-types](01M61A10Z27NFHK3KJ5VBEAVC3.md)"
hasLead: "[Gordian Dziwis](3f1909e2-5284-4d40-a1ed-8bf222238d03.md)"
hasIntent: "RDF Trilith terms interpret into resources of a type the interpretation chooses, so one IRI reads as a term, a graph or a shape."
hasInput:
  - "[Software rdf-types](01M61A10QZA7DMKQGPFMH9QGKT.md)"
  - "[Software rdf-syntax](01M61A10SJEHPWGSSTEG7DKD1Y.md)"
hasOutput:
  - "[Software RDF Trilith](20260830114137.md)"
vetting: unread
---

Draft the interpretation trait as a code skeleton.
Implement it in RDF Trilith.

## Follows

<!-- dataview-serializer-js:
const current = dv.current();
const pages = dv.pages();
const refers = (value, id) => String(value).includes(id + ".md");
const places = pages.where(page => refers(page.type, "20260608023514")).array();
const outputs = (act, place) => refers(act.hasOutputPlace, place.file.name) || refers(act.hasOutputMapPlace, place.file.name) || refers(act.hasOutput, place.file.name)
  || refers(place.isOutputPlaceOf, act.file.name) || refers(place.isOutputOf, act.file.name);
const inputs = (act, place) => refers(act.hasInputPlace, place.file.name) || refers(act.hasInputCollectPlace, place.file.name) || refers(act.hasInput, place.file.name)
  || refers(place.isInputPlaceOf, act.file.name) || refers(place.isInputOf, act.file.name);
const follows = (later, earlier) => refers(later.follows, earlier.file.name)
  || refers(earlier.precedes, later.file.name)
  || places.some(place => outputs(earlier, place) && inputs(later, place));
dv.paragraph(pages
  .where(page => follows(current, page))
  .sort(page => page.file.aliases[0])
  .map(page => "- [" + page.file.aliases[0] + "](" + page.file.name + ".md)" + (page.hasState ? ": " + String(page.hasState).toLowerCase() : ""))
  .join("\n"));
 -->

## Precedes

<!-- dataview-serializer-js:
const current = dv.current();
const pages = dv.pages();
const refers = (value, id) => String(value).includes(id + ".md");
const places = pages.where(page => refers(page.type, "20260608023514")).array();
const outputs = (act, place) => refers(act.hasOutputPlace, place.file.name) || refers(act.hasOutputMapPlace, place.file.name) || refers(act.hasOutput, place.file.name)
  || refers(place.isOutputPlaceOf, act.file.name) || refers(place.isOutputOf, act.file.name);
const inputs = (act, place) => refers(act.hasInputPlace, place.file.name) || refers(act.hasInputCollectPlace, place.file.name) || refers(act.hasInput, place.file.name)
  || refers(place.isInputPlaceOf, act.file.name) || refers(place.isInputOf, act.file.name);
const follows = (later, earlier) => refers(later.follows, earlier.file.name)
  || refers(earlier.precedes, later.file.name)
  || places.some(place => outputs(earlier, place) && inputs(later, place));
dv.paragraph(pages
  .where(page => follows(page, current))
  .sort(page => page.file.aliases[0])
  .map(page => "- [" + page.file.aliases[0] + "](" + page.file.name + ".md)" + (page.hasState ? ": " + String(page.hasState).toLowerCase() : ""))
  .join("\n"));
 -->
<!-- dataview-serializer-js-result -->
<!-- dataview-serializer-js-result-end -->
