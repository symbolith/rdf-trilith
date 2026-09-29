use rdf_trilith_term::{Object, ObjectBuf, Predicate, PredicateBuf, Subject, SubjectBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Triple<'g> {
    pub subject: Subject<'g>,
    pub predicate: Predicate<'g>,
    pub object: Object<'g>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TripleBuf {
    pub subject: SubjectBuf,
    pub predicate: PredicateBuf,
    pub object: ObjectBuf,
}

impl<'g> Triple<'g> {
    pub fn new(
        subject: impl Into<Subject<'g>>,
        predicate: impl Into<Predicate<'g>>,
        object: impl Into<Object<'g>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }
}

impl TripleBuf {
    pub fn new(
        subject: impl Into<SubjectBuf>,
        predicate: impl Into<PredicateBuf>,
        object: impl Into<ObjectBuf>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }
}

impl<'g, S, P, O> From<(S, P, O)> for Triple<'g>
where
    S: Into<Subject<'g>>,
    P: Into<Predicate<'g>>,
    O: Into<Object<'g>>,
{
    fn from((subject, predicate, object): (S, P, O)) -> Self {
        Self::new(subject, predicate, object)
    }
}

impl<S, P, O> From<(S, P, O)> for TripleBuf
where
    S: Into<SubjectBuf>,
    P: Into<PredicateBuf>,
    O: Into<ObjectBuf>,
{
    fn from((subject, predicate, object): (S, P, O)) -> Self {
        Self::new(subject, predicate, object)
    }
}

impl<'g> From<&'g TripleBuf> for Triple<'g> {
    fn from(triple: &'g TripleBuf) -> Self {
        Self {
            subject: (&triple.subject).into(),
            predicate: triple.predicate.as_ref(),
            object: (&triple.object).into(),
        }
    }
}

impl From<Triple<'_>> for TripleBuf {
    fn from(triple: Triple<'_>) -> Self {
        Self {
            subject: triple.subject.into(),
            predicate: triple.predicate.to_owned(),
            object: triple.object.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rdf_trilith_term::{Iri, IriBuf, LexicalFormBuf, Literal};

    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn iri_buf(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    fn simple(text: &str) -> Literal {
        Literal::Simple {
            lexical_form: LexicalFormBuf::new(text).unwrap(),
        }
    }

    #[test]
    fn borrowed_from_references() {
        let literal = simple("hello");
        let triple = Triple::new(iri("http://x/s"), iri("http://x/p"), &literal);
        assert_eq!(triple.subject.as_iri(), Some(iri("http://x/s")));
        assert_eq!(triple.object.as_literal(), Some(&literal));
    }

    #[test]
    fn buf_from_owned_moves() {
        let triple = TripleBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            simple("hello"),
        );
        assert!(matches!(triple.subject, SubjectBuf::Iri(_)));
        assert!(matches!(triple.object, ObjectBuf::Literal(_)));
    }

    #[test]
    fn tuples_convert() {
        let borrowed: Triple<'_> = (iri("http://x/s"), iri("http://x/p"), iri("http://x/o")).into();
        let owned: TripleBuf = (iri_buf("http://x/s"), iri_buf("http://x/p"), simple("hi")).into();
        assert_eq!(borrowed.predicate, iri("http://x/p"));
        assert!(matches!(owned.object, ObjectBuf::Literal(_)));
    }

    #[test]
    fn ownership_bridge_round_trip() {
        let owned = TripleBuf::new(
            iri_buf("http://x/s"),
            iri_buf("http://x/p"),
            simple("hello"),
        );
        let borrowed = Triple::from(&owned);
        assert_eq!(TripleBuf::from(borrowed), owned);
    }

    #[test]
    fn borrowed_is_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Triple<'_>>();
    }
}
