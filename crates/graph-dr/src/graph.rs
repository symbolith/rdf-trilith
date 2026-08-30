use std::collections::HashSet;

use crate::triple::{Triple, TripleBuf};

pub trait Graph {
    type Triple<'g>
    where
        Self: 'g;
    fn triples(&self) -> impl Iterator<Item = Self::Triple<'_>>;
    fn contains<'s>(&'s self, triple: Self::Triple<'s>) -> bool;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait GraphMut: Graph {
    type TripleBuf;
    fn insert(&mut self, triple: impl Into<Self::TripleBuf>) -> bool;
    fn remove<'s>(&'s mut self, triple: Self::Triple<'s>) -> bool;
}

impl<Inner: Graph> Graph for &Inner {
    type Triple<'g>
        = Inner::Triple<'g>
    where
        Self: 'g;

    fn triples(&self) -> impl Iterator<Item = Self::Triple<'_>> {
        (**self).triples()
    }

    fn contains<'s>(&'s self, triple: Self::Triple<'s>) -> bool {
        (**self).contains(triple)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<Inner: Graph> Graph for &mut Inner {
    type Triple<'g>
        = Inner::Triple<'g>
    where
        Self: 'g;

    fn triples(&self) -> impl Iterator<Item = Self::Triple<'_>> {
        (**self).triples()
    }

    fn contains<'s>(&'s self, triple: Self::Triple<'s>) -> bool {
        (**self).contains(triple)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<Inner: GraphMut> GraphMut for &mut Inner {
    type TripleBuf = Inner::TripleBuf;

    fn insert(&mut self, triple: impl Into<Self::TripleBuf>) -> bool {
        (**self).insert(triple)
    }

    fn remove<'s>(&'s mut self, triple: Self::Triple<'s>) -> bool {
        (**self).remove(triple)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryGraph(HashSet<TripleBuf>);

impl MemoryGraph {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl Graph for MemoryGraph {
    type Triple<'g> = Triple<'g>;

    fn triples(&self) -> impl Iterator<Item = Triple<'_>> {
        self.0.iter().map(Triple::from)
    }

    fn contains<'s>(&'s self, triple: Triple<'s>) -> bool {
        self.0.contains(&TripleBuf::from(triple))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl GraphMut for MemoryGraph {
    type TripleBuf = TripleBuf;

    fn insert(&mut self, triple: impl Into<TripleBuf>) -> bool {
        self.0.insert(triple.into())
    }

    fn remove<'s>(&'s mut self, triple: Triple<'s>) -> bool {
        self.0.remove(&TripleBuf::from(triple))
    }
}

#[cfg(test)]
mod tests {
    use term_dr::{Iri, IriBuf};

    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn iri_buf(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    fn example() -> TripleBuf {
        TripleBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        )
    }

    #[test]
    fn insert_owned_borrowed_and_tuple() {
        let mut graph = MemoryGraph::new();
        assert!(graph.insert(example()));
        assert!(!graph.insert(Triple::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
        )));
        assert!(!graph.insert((
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        )));
        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn contains_and_remove() {
        let mut graph = MemoryGraph::new();
        graph.insert(example());
        let owned = example();
        assert!(graph.contains(Triple::from(&owned)));
        assert!(graph.remove(Triple::from(&owned)));
        assert!(graph.is_empty());
    }

    #[test]
    fn triples_iterate_borrowed() {
        let mut graph = MemoryGraph::new();
        graph.insert(example());
        let owned = example();
        let borrowed: Vec<Triple<'_>> = graph.triples().collect();
        assert_eq!(borrowed, vec![Triple::from(&owned)]);
    }

    #[test]
    fn generic_graph_bound_accepts_memory_and_reference() {
        fn count(graph: impl Graph) -> usize {
            graph.len()
        }
        let mut graph = MemoryGraph::new();
        graph.insert(example());
        assert_eq!(count(&graph), 1);
        assert_eq!(count(&mut graph), 1);
        assert_eq!(count(graph), 1);
    }

    #[test]
    fn references_delegate() {
        let mut graph = MemoryGraph::new();
        let owned = example();
        {
            let mut mutable = &mut graph;
            assert!(GraphMut::insert(&mut mutable, example()));
            assert!(GraphMut::remove(&mut mutable, Triple::from(&owned)));
            assert!(GraphMut::insert(&mut mutable, example()));
            assert!(Graph::contains(&mutable, Triple::from(&owned)));
        }
        let shared = &graph;
        assert!(Graph::contains(&shared, Triple::from(&owned)));
        assert_eq!(Graph::triples(&shared).count(), 1);
        assert_eq!(Graph::len(&shared), 1);
    }

    #[test]
    fn borrowed_store_implements_graph() {
        struct BorrowedGraph<'a>(HashSet<Triple<'a>>);

        impl<'a> Graph for BorrowedGraph<'a> {
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

        impl<'a> GraphMut for BorrowedGraph<'a> {
            type TripleBuf = Triple<'a>;

            fn insert(&mut self, triple: impl Into<Triple<'a>>) -> bool {
                self.0.insert(triple.into())
            }

            fn remove(&mut self, triple: Triple<'a>) -> bool {
                self.0.remove(&triple)
            }
        }

        let subject = iri_buf("http://x/s");
        let predicate = iri_buf("http://x/p");
        let object = iri_buf("http://x/o");
        let mut graph = BorrowedGraph(HashSet::new());
        assert!(graph.insert((&*subject, &*predicate, &*object)));
        assert!(graph.contains(Triple::new(&*subject, &*predicate, &*object)));
        assert_eq!(graph.triples().count(), 1);
    }
}
