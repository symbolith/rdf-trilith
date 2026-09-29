use rdf_trilith_graph::{Triple, TripleBuf};
use rdf_trilith_term::{Object, ObjectBuf, Predicate, PredicateBuf, Subject, SubjectBuf};

use crate::graph_name::{GraphName, GraphNameBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Quad<'d> {
    pub subject: Subject<'d>,
    pub predicate: Predicate<'d>,
    pub object: Object<'d>,
    pub graph_name: Option<GraphName<'d>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuadBuf {
    pub subject: SubjectBuf,
    pub predicate: PredicateBuf,
    pub object: ObjectBuf,
    pub graph_name: Option<GraphNameBuf>,
}

impl<'d> Quad<'d> {
    pub fn new(
        subject: impl Into<Subject<'d>>,
        predicate: impl Into<Predicate<'d>>,
        object: impl Into<Object<'d>>,
        graph_name: impl Into<Option<GraphName<'d>>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name: graph_name.into(),
        }
    }

    pub fn triple(&self) -> Triple<'d> {
        Triple {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object,
        }
    }
}

impl QuadBuf {
    pub fn new(
        subject: impl Into<SubjectBuf>,
        predicate: impl Into<PredicateBuf>,
        object: impl Into<ObjectBuf>,
        graph_name: impl Into<Option<GraphNameBuf>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name: graph_name.into(),
        }
    }

    pub fn into_triple(self) -> TripleBuf {
        TripleBuf {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object,
        }
    }
}

impl<'d, S, P, O> From<(S, P, O)> for Quad<'d>
where
    S: Into<Subject<'d>>,
    P: Into<Predicate<'d>>,
    O: Into<Object<'d>>,
{
    fn from((subject, predicate, object): (S, P, O)) -> Self {
        Self::new(subject, predicate, object, None)
    }
}

impl<S, P, O> From<(S, P, O)> for QuadBuf
where
    S: Into<SubjectBuf>,
    P: Into<PredicateBuf>,
    O: Into<ObjectBuf>,
{
    fn from((subject, predicate, object): (S, P, O)) -> Self {
        Self::new(subject, predicate, object, None)
    }
}

impl<'d, S, P, O, G> From<(S, P, O, G)> for Quad<'d>
where
    S: Into<Subject<'d>>,
    P: Into<Predicate<'d>>,
    O: Into<Object<'d>>,
    G: Into<Option<GraphName<'d>>>,
{
    fn from((subject, predicate, object, graph_name): (S, P, O, G)) -> Self {
        Self::new(subject, predicate, object, graph_name)
    }
}

impl<S, P, O, G> From<(S, P, O, G)> for QuadBuf
where
    S: Into<SubjectBuf>,
    P: Into<PredicateBuf>,
    O: Into<ObjectBuf>,
    G: Into<Option<GraphNameBuf>>,
{
    fn from((subject, predicate, object, graph_name): (S, P, O, G)) -> Self {
        Self::new(subject, predicate, object, graph_name)
    }
}

impl<'d> From<Triple<'d>> for Quad<'d> {
    fn from(triple: Triple<'d>) -> Self {
        Self {
            subject: triple.subject,
            predicate: triple.predicate,
            object: triple.object,
            graph_name: None,
        }
    }
}

impl From<TripleBuf> for QuadBuf {
    fn from(triple: TripleBuf) -> Self {
        Self {
            subject: triple.subject,
            predicate: triple.predicate,
            object: triple.object,
            graph_name: None,
        }
    }
}

impl<'d> From<&'d QuadBuf> for Quad<'d> {
    fn from(quad: &'d QuadBuf) -> Self {
        Self {
            subject: (&quad.subject).into(),
            predicate: quad.predicate.as_ref(),
            object: (&quad.object).into(),
            graph_name: quad.graph_name.as_ref().map(Into::into),
        }
    }
}

impl From<Quad<'_>> for QuadBuf {
    fn from(quad: Quad<'_>) -> Self {
        Self {
            subject: quad.subject.into(),
            predicate: quad.predicate.to_owned(),
            object: quad.object.into(),
            graph_name: quad.graph_name.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use rdf_trilith_term::{Iri, IriBuf};

    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn iri_buf(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    #[test]
    fn triple_enters_default_graph() {
        let quad = Quad::from(Triple::new(
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
        ));
        assert_eq!(quad.graph_name, None);
        let quad_buf = QuadBuf::from(TripleBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
        ));
        assert_eq!(quad_buf.graph_name, None);
    }

    #[test]
    fn tuples_convert() {
        let default_graph: Quad<'_> =
            (iri("http://x/s"), iri("http://x/p"), iri("http://x/o")).into();
        assert_eq!(default_graph.graph_name, None);
        let named: Quad<'_> = (
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            GraphName::from(iri("http://x/g")),
        )
            .into();
        assert_eq!(named.graph_name, Some(GraphName::from(iri("http://x/g"))));
        let named_buf: QuadBuf = (
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            GraphNameBuf::from(iri_buf("http://x/g")),
        )
            .into();
        assert!(named_buf.graph_name.is_some());
    }

    #[test]
    fn four_tuple_with_explicit_none() {
        let borrowed: Quad<'_> = (
            iri("http://x/s"),
            iri("http://x/p"),
            iri("http://x/o"),
            None::<GraphName<'_>>,
        )
            .into();
        assert_eq!(borrowed.graph_name, None);
        let owned: QuadBuf = (
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            None::<GraphNameBuf>,
        )
            .into();
        assert_eq!(owned.graph_name, None);
    }

    #[test]
    fn ownership_bridge_round_trip() {
        let owned = QuadBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            iri_buf("http://x/o"),
            GraphNameBuf::from(iri_buf("http://x/g")),
        );
        let borrowed = Quad::from(&owned);
        assert_eq!(QuadBuf::from(borrowed), owned);
        assert_eq!(borrowed.triple().predicate, iri("http://x/p"));
        assert_eq!(owned.clone().into_triple().predicate, iri_buf("http://x/p"));
    }
}
