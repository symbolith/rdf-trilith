use rdf_trilith_macro_enum::morphism;
use rdf_trilith_term::{
    BlankNode, BlankNodeBuf, Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized,
    IriNormalizedBuf, Subject, SubjectBuf, Term, TermBuf,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphName<'d> {
    Iri(&'d Iri),
    BlankNode(&'d BlankNode),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphNameBuf {
    Iri(IriBuf),
    BlankNode(BlankNodeBuf),
}

morphism!(GraphName { Iri, BlankNode });
morphism!(IriNormalized as Iri => GraphName);
morphism!(IriAbsolute as Iri => GraphName);
morphism!(Subject => GraphName { Iri, BlankNode });

impl<'d> From<GraphName<'d>> for Subject<'d> {
    fn from(graph_name: GraphName<'d>) -> Self {
        match graph_name {
            GraphName::Iri(iri) => Self::Iri(iri),
            GraphName::BlankNode(blank_node) => Self::BlankNode(blank_node),
        }
    }
}

impl From<GraphNameBuf> for SubjectBuf {
    fn from(graph_name: GraphNameBuf) -> Self {
        match graph_name {
            GraphNameBuf::Iri(iri) => Self::Iri(iri),
            GraphNameBuf::BlankNode(blank_node) => Self::BlankNode(blank_node),
        }
    }
}

impl<'d> From<GraphName<'d>> for Term<'d> {
    fn from(graph_name: GraphName<'d>) -> Self {
        match graph_name {
            GraphName::Iri(iri) => Self::Iri(iri),
            GraphName::BlankNode(blank_node) => Self::BlankNode(blank_node),
        }
    }
}

impl From<GraphNameBuf> for TermBuf {
    fn from(graph_name: GraphNameBuf) -> Self {
        match graph_name {
            GraphNameBuf::Iri(iri) => Self::Iri(iri),
            GraphNameBuf::BlankNode(blank_node) => Self::BlankNode(blank_node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn iri_buf(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    #[test]
    fn widens_from_iri_families() {
        let plain = GraphName::from(iri("http://x/g"));
        assert_eq!(plain.as_iri(), Some(iri("http://x/g")));
        let normalized = IriNormalized::new("http://x/g").unwrap();
        assert_eq!(GraphName::from(normalized), plain);
        let absolute = IriAbsolute::new("http://x/g").unwrap();
        assert_eq!(GraphName::from(absolute), plain);
        assert!(matches!(
            GraphNameBuf::from(IriNormalizedBuf::new("http://x/g").unwrap()),
            GraphNameBuf::Iri(_)
        ));
        assert!(matches!(
            GraphNameBuf::from(IriAbsoluteBuf::new("http://x/g").unwrap()),
            GraphNameBuf::Iri(_)
        ));
    }

    #[test]
    fn subject_morphisms_both_ways() {
        let graph_name = GraphName::from(iri("http://x/g"));
        let subject = Subject::from(graph_name);
        assert_eq!(GraphName::from(subject), graph_name);
        assert_eq!(graph_name.as_subject(), Some(subject));
        let graph_name_buf = GraphNameBuf::from(iri_buf("http://x/g"));
        let subject_buf = SubjectBuf::from(graph_name_buf.clone());
        assert_eq!(GraphNameBuf::from(subject_buf), graph_name_buf);
    }

    #[test]
    fn widens_to_term() {
        let graph_name = GraphName::from(iri("http://x/g"));
        assert_eq!(Term::from(graph_name), Term::Iri(iri("http://x/g")));
        assert!(matches!(
            TermBuf::from(GraphNameBuf::from(iri_buf("http://x/g"))),
            TermBuf::Iri(_)
        ));
    }

    #[test]
    fn blank_node_round_trip() {
        let blank_node_buf = BlankNodeBuf::new("b0").unwrap();
        let graph_name = GraphName::from(&*blank_node_buf);
        assert_eq!(
            graph_name.as_blank_node(),
            Some(BlankNode::new("b0").unwrap())
        );
        assert_eq!(graph_name.as_iri(), None);
        let owned = GraphNameBuf::from(graph_name);
        assert_eq!(
            owned.clone().into_blank_node(),
            Some(BlankNodeBuf::new("b0").unwrap())
        );
        assert_eq!(owned.into_iri(), None);
    }

    #[test]
    fn buf_narrows_into_subject() {
        let graph_name_buf = GraphNameBuf::from(iri_buf("http://x/g"));
        assert!(matches!(
            graph_name_buf.into_subject(),
            Some(SubjectBuf::Iri(_))
        ));
    }
}
