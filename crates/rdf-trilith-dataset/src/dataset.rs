use std::collections::HashMap;

use rdf_trilith_graph::{Graph, GraphMut, MemoryGraph, Triple, TripleBuf};

use crate::graph_name::{GraphName, GraphNameBuf};
use crate::quad::{Quad, QuadBuf};

pub trait Dataset {
    type Triple<'d>
    where
        Self: 'd;
    type Quad<'d>
    where
        Self: 'd;
    type Graph<'d>: Graph<Triple<'d> = Self::Triple<'d>>
    where
        Self: 'd;

    fn quads(&self) -> impl Iterator<Item = Self::Quad<'_>>;
    fn graph(&self, graph_name: Option<GraphName<'_>>) -> Option<&Self::Graph<'_>>;
    fn graph_names(&self) -> impl Iterator<Item = GraphName<'_>>;
    fn contains<'s>(&'s self, quad: Self::Quad<'s>) -> bool;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait DatasetMut: Dataset {
    type QuadBuf;
    fn insert(&mut self, quad: impl Into<Self::QuadBuf>) -> bool;
    fn remove<'s>(&'s mut self, quad: Self::Quad<'s>) -> bool;
}

impl<Inner: Dataset> Dataset for &Inner {
    type Triple<'d>
        = Inner::Triple<'d>
    where
        Self: 'd;
    type Quad<'d>
        = Inner::Quad<'d>
    where
        Self: 'd;
    type Graph<'d>
        = Inner::Graph<'d>
    where
        Self: 'd;

    fn quads(&self) -> impl Iterator<Item = Self::Quad<'_>> {
        (**self).quads()
    }

    fn graph(&self, graph_name: Option<GraphName<'_>>) -> Option<&Self::Graph<'_>> {
        (**self).graph(graph_name)
    }

    fn graph_names(&self) -> impl Iterator<Item = GraphName<'_>> {
        (**self).graph_names()
    }

    fn contains<'s>(&'s self, quad: Self::Quad<'s>) -> bool {
        (**self).contains(quad)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<Inner: Dataset> Dataset for &mut Inner {
    type Triple<'d>
        = Inner::Triple<'d>
    where
        Self: 'd;
    type Quad<'d>
        = Inner::Quad<'d>
    where
        Self: 'd;
    type Graph<'d>
        = Inner::Graph<'d>
    where
        Self: 'd;

    fn quads(&self) -> impl Iterator<Item = Self::Quad<'_>> {
        (**self).quads()
    }

    fn graph(&self, graph_name: Option<GraphName<'_>>) -> Option<&Self::Graph<'_>> {
        (**self).graph(graph_name)
    }

    fn graph_names(&self) -> impl Iterator<Item = GraphName<'_>> {
        (**self).graph_names()
    }

    fn contains<'s>(&'s self, quad: Self::Quad<'s>) -> bool {
        (**self).contains(quad)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<Inner: DatasetMut> DatasetMut for &mut Inner {
    type QuadBuf = Inner::QuadBuf;

    fn insert(&mut self, quad: impl Into<Self::QuadBuf>) -> bool {
        (**self).insert(quad)
    }

    fn remove<'s>(&'s mut self, quad: Self::Quad<'s>) -> bool {
        (**self).remove(quad)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryDataset(HashMap<Option<GraphNameBuf>, MemoryGraph>);

impl MemoryDataset {
    pub fn new() -> Self {
        Self(HashMap::from([(None, MemoryGraph::new())]))
    }
}

impl Default for MemoryDataset {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MemoryGraph> for MemoryDataset {
    fn from(graph: MemoryGraph) -> Self {
        Self(HashMap::from([(None, graph)]))
    }
}

impl Dataset for MemoryDataset {
    type Triple<'d> = Triple<'d>;
    type Quad<'d> = Quad<'d>;
    type Graph<'d> = MemoryGraph;

    fn quads(&self) -> impl Iterator<Item = Quad<'_>> {
        self.0.iter().flat_map(|(graph_name, graph)| {
            graph.triples().map(move |triple| Quad {
                subject: triple.subject,
                predicate: triple.predicate,
                object: triple.object,
                graph_name: graph_name.as_ref().map(Into::into),
            })
        })
    }

    fn graph(&self, graph_name: Option<GraphName<'_>>) -> Option<&MemoryGraph> {
        self.0.get(&graph_name.map(GraphNameBuf::from))
    }

    fn graph_names(&self) -> impl Iterator<Item = GraphName<'_>> {
        self.0.keys().flatten().map(Into::into)
    }

    fn contains<'s>(&'s self, quad: Quad<'s>) -> bool {
        self.graph(quad.graph_name)
            .is_some_and(|graph| graph.contains(quad.triple()))
    }

    fn len(&self) -> usize {
        self.0.values().map(Graph::len).sum()
    }
}

impl DatasetMut for MemoryDataset {
    type QuadBuf = QuadBuf;

    fn insert(&mut self, quad: impl Into<QuadBuf>) -> bool {
        let QuadBuf {
            subject,
            predicate,
            object,
            graph_name,
        } = quad.into();
        self.0
            .entry(graph_name)
            .or_default()
            .insert(TripleBuf::new(subject, predicate, object))
    }

    fn remove<'s>(&'s mut self, quad: Quad<'s>) -> bool {
        let graph_name = quad.graph_name.map(GraphNameBuf::from);
        let Some(graph) = self.0.get_mut(&graph_name) else {
            return false;
        };
        let removed = graph.remove(quad.triple());
        if removed && graph.is_empty() && graph_name.is_some() {
            self.0.remove(&graph_name);
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rdf_trilith_term::{BlankNodeBuf, Iri, IriBuf};

    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn iri_buf(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    fn named(graph: &str) -> GraphNameBuf {
        GraphNameBuf::from(iri_buf(graph))
    }

    fn dataset() -> MemoryDataset {
        let mut dataset = MemoryDataset::new();
        dataset.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        ));
        dataset.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            named("http://x/g"),
        ));
        dataset
    }

    #[test]
    fn insert_separates_graphs() {
        let dataset = dataset();
        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.graph(None).unwrap().len(), 1);
        let name = GraphName::from(iri("http://x/g"));
        assert_eq!(dataset.graph(Some(name)).unwrap().len(), 1);
        assert!(
            dataset
                .graph(Some(GraphName::from(iri("http://x/other"))))
                .is_none()
        );
    }

    #[test]
    fn quads_reassemble_borrowed() {
        let dataset = dataset();
        let quads: Vec<Quad<'_>> = dataset.quads().collect();
        assert_eq!(quads.len(), 2);
        assert!(quads.iter().any(|quad| quad.graph_name.is_none()));
        assert!(
            quads
                .iter()
                .any(|quad| quad.graph_name == Some(GraphName::from(iri("http://x/g"))))
        );
    }

    #[test]
    fn graph_names_lists_named_graphs_only() {
        let dataset = dataset();
        let names: Vec<GraphName<'_>> = dataset.graph_names().collect();
        assert_eq!(names, vec![GraphName::from(iri("http://x/g"))]);
    }

    #[test]
    fn contains_by_borrowed() {
        let dataset = dataset();
        let in_default = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            None,
        );
        assert!(dataset.contains(in_default));
        let in_named = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            GraphName::from(iri("http://x/g")),
        );
        assert!(dataset.contains(in_named));
        let absent = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/s"),
            None,
        );
        assert!(!dataset.contains(absent));
    }

    #[test]
    fn remove_drops_empty_graphs() {
        let mut dataset = dataset();
        let in_named = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            GraphName::from(iri("http://x/g")),
        );
        assert!(dataset.remove(in_named));
        assert!(!dataset.remove(in_named));
        assert_eq!(dataset.graph_names().count(), 0);
        assert_eq!(dataset.len(), 1);
    }

    #[test]
    fn borrowed_quad_round_trips_through_insert() {
        let mut dataset = MemoryDataset::new();
        let quad = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            GraphName::from(iri("http://x/g")),
        );
        assert!(dataset.insert(quad));
        assert!(dataset.contains(quad));
    }

    #[test]
    fn blank_node_named_graph() {
        let mut dataset = MemoryDataset::new();
        let blank_node_buf = BlankNodeBuf::new("b0").unwrap();
        dataset.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            GraphNameBuf::from(blank_node_buf.clone()),
        ));
        let name = GraphName::from(&*blank_node_buf);
        assert!(dataset.graph(Some(name)).is_some());
        assert_eq!(dataset.graph_names().collect::<Vec<_>>(), vec![name]);
        assert!(dataset.contains(Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            name,
        )));
    }

    #[test]
    fn default_graph_always_exists() {
        let mut dataset = MemoryDataset::new();
        assert!(dataset.graph(None).unwrap().is_empty());
        dataset.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        ));
        let quad = Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            None,
        );
        assert!(dataset.remove(quad));
        assert!(dataset.is_empty());
        assert!(dataset.graph(None).unwrap().is_empty());
    }

    #[test]
    fn graph_widens_into_dataset() {
        let mut graph = MemoryGraph::new();
        graph.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        ));
        let dataset = MemoryDataset::from(graph);
        assert_eq!(dataset.len(), 1);
        assert_eq!(dataset.graph_names().count(), 0);
        assert!(dataset.contains(Quad::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            None,
        )));
    }

    #[test]
    fn references_delegate() {
        let mut dataset = MemoryDataset::new();
        let quad = QuadBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            named("http://x/g"),
        );
        let borrowed = Quad::from(&quad);
        {
            let mut mutable = &mut dataset;
            assert!(DatasetMut::insert(&mut mutable, borrowed));
            assert!(DatasetMut::remove(&mut mutable, borrowed));
            assert!(DatasetMut::insert(&mut mutable, borrowed));
            assert!(Dataset::contains(&mutable, borrowed));
        }
        let shared = &dataset;
        assert!(Dataset::contains(&shared, borrowed));
        assert_eq!(Dataset::quads(&shared).count(), 1);
        assert_eq!(Dataset::graph_names(&shared).count(), 1);
        assert_eq!(Dataset::len(&shared), 1);
    }

    #[test]
    fn generic_code_drills_into_graphs() {
        fn lookup<'s, D>(dataset: &'s D, quad: Quad<'s>) -> bool
        where
            D: Dataset<Triple<'s> = Triple<'s>, Quad<'s> = Quad<'s>>,
        {
            let Some(graph) = dataset.graph(quad.graph_name) else {
                return false;
            };
            graph.contains(quad.triple())
        }

        let dataset = dataset();
        assert!(lookup(
            &dataset,
            Quad::new(
                iri("http://x/s"),
                iri("http://x/p"),
                iri("http://x/o"),
                GraphName::from(iri("http://x/g")),
            ),
        ));
        assert!(!lookup(
            &dataset,
            Quad::new(
                iri("http://x/s"),
                iri("http://x/p"),
                iri("http://x/o"),
                GraphName::from(iri("http://x/other")),
            ),
        ));
    }

    #[test]
    fn borrowed_store_implements_dataset() {
        struct BorrowedTriples<'a>(HashSet<Triple<'a>>);

        impl<'a> Graph for BorrowedTriples<'a> {
            type Triple<'g>
                = Triple<'a>
            where
                Self: 'g;

            fn triples(&self) -> impl Iterator<Item = Triple<'a>> {
                self.0.iter().copied()
            }

            fn contains(&self, triple: Triple<'a>) -> bool {
                self.0.contains(&triple)
            }

            fn len(&self) -> usize {
                self.0.len()
            }
        }

        struct BorrowedDataset<'a>(BorrowedTriples<'a>);

        impl<'a> Dataset for BorrowedDataset<'a> {
            type Triple<'d>
                = Triple<'a>
            where
                Self: 'd;
            type Quad<'d>
                = Quad<'a>
            where
                Self: 'd;
            type Graph<'d>
                = BorrowedTriples<'a>
            where
                Self: 'd;

            fn quads(&self) -> impl Iterator<Item = Quad<'a>> {
                self.0.triples().map(Quad::from)
            }

            fn graph(&self, graph_name: Option<GraphName<'_>>) -> Option<&BorrowedTriples<'a>> {
                graph_name.is_none().then_some(&self.0)
            }

            fn graph_names(&self) -> impl Iterator<Item = GraphName<'_>> {
                std::iter::empty()
            }

            fn contains(&self, quad: Quad<'a>) -> bool {
                quad.graph_name.is_none() && self.0.contains(quad.triple())
            }

            fn len(&self) -> usize {
                self.0.len()
            }
        }

        let subject = iri_buf("http://x/s");
        let predicate = iri_buf("http://x/p");
        let object = iri_buf("http://x/o");
        let triple = Triple::new(&*subject, &*predicate, &*object);
        let dataset = BorrowedDataset(BorrowedTriples(HashSet::from([triple])));
        assert_eq!(dataset.len(), 1);
        assert!(dataset.contains(Quad::from(triple)));
        assert_eq!(dataset.quads().count(), 1);
        assert!(dataset.graph(None).is_some());
    }
}
