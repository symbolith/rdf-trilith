mod compat;
mod error;
mod position;
mod term;

#[cfg(feature = "oxrdf")]
pub use compat::ConversionError;
pub use error::ValidationError;
pub use rdf_trilith_iri::{Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized, IriNormalizedBuf};
pub use position::{Object, ObjectBuf, Predicate, PredicateBuf, Subject, SubjectBuf};
pub use term::{
    BlankNode, BlankNodeBuf, LanguageTag, LanguageTagBuf, LexicalForm, LexicalFormBuf, Literal,
    LiteralBuf, Term, TermBuf,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn iri(text: &str) -> &Iri {
        Iri::new(text).unwrap()
    }

    fn english(text: &str) -> Literal {
        Literal::LanguageTagged {
            lexical_form: LexicalFormBuf::new(text).unwrap(),
            language_tag: LanguageTagBuf::new("en").unwrap(),
        }
    }

    #[test]
    fn borrowed_enums_are_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Subject<'_>>();
        assert_copy::<Object<'_>>();
        assert_copy::<Term<'_>>();
    }

    #[test]
    fn borrowed_enum_from_anywhere() {
        let iri_buf = IriBuf::new("http://example.com/alice").unwrap();
        let from_buf = Subject::Iri(&iri_buf);
        let standalone = Iri::new("http://example.com/alice").unwrap();
        let from_borrowed = Subject::Iri(standalone);
        assert_eq!(from_buf, from_borrowed);
    }

    #[test]
    fn widen_borrowed() {
        let subject = Subject::from(iri("http://example.com/a"));
        assert!(matches!(subject, Subject::Iri(_)));
        let literal = english("hi");
        let object = Object::from(&literal);
        assert!(matches!(object, Object::Literal(_)));
    }

    #[test]
    fn widen_owned() {
        let subject_buf = SubjectBuf::from(IriBuf::new("http://example.com/a").unwrap());
        assert!(matches!(subject_buf, SubjectBuf::Iri(_)));
        let term_buf = TermBuf::from(english("hi"));
        assert!(matches!(term_buf, TermBuf::Literal(_)));
    }

    #[test]
    fn narrow_borrowed() {
        let subject = Subject::from(iri("http://example.com/a"));
        assert_eq!(subject.as_iri(), Some(iri("http://example.com/a")));
        assert_eq!(subject.as_blank_node(), None);
    }

    #[test]
    fn narrow_owned() {
        let subject_buf = SubjectBuf::from(IriBuf::new("http://example.com/a").unwrap());
        assert_eq!(
            subject_buf.into_iri(),
            Some(IriBuf::new("http://example.com/a").unwrap()),
        );
        assert_eq!(TermBuf::from(english("hi")).into_blank_node(), None);
    }

    #[test]
    fn widen_between_enums() {
        let subject = Subject::from(iri("http://example.com/a"));
        let via_object = Term::from(Object::from(subject));
        let direct = Term::from(subject);
        assert_eq!(via_object, direct);
    }

    #[test]
    fn narrow_between_enums() {
        let literal = english("hi");
        let term = Term::from(&literal);
        assert_eq!(term.as_subject(), None);
        assert!(term.as_object().is_some());
        let term_buf = TermBuf::from(english("hi"));
        assert_eq!(term_buf.into_subject(), None);
    }

    #[test]
    fn widen_normalized_iri() {
        let normalized = IriNormalized::new("http://example.com/a").unwrap();
        let subject = Subject::from(normalized);
        assert_eq!(subject, Subject::from(iri("http://example.com/a")));
        let subject_buf = SubjectBuf::from(IriNormalizedBuf::new("http://example.com/a").unwrap());
        assert!(matches!(subject_buf, SubjectBuf::Iri(_)));
    }

    #[test]
    fn widen_absolute_iri() {
        let absolute = IriAbsolute::new("http://example.com/a").unwrap();
        let subject = Subject::from(absolute);
        assert_eq!(subject, Subject::from(iri("http://example.com/a")));
        let subject_buf = SubjectBuf::from(IriAbsoluteBuf::new("http://example.com/a").unwrap());
        assert!(matches!(subject_buf, SubjectBuf::Iri(_)));
    }

    #[test]
    fn ownership_bridge_round_trip() {
        let subject_buf = SubjectBuf::from(BlankNodeBuf::new("b0").unwrap());
        let subject = Subject::from(&subject_buf);
        assert_eq!(SubjectBuf::from(subject), subject_buf);
        let object_buf = ObjectBuf::from(english("hi"));
        let object = Object::from(&object_buf);
        assert_eq!(ObjectBuf::from(object), object_buf);
    }
}
