//! Conversions between [`oxrdf`] terms and term-dr types.
//!
//! - `oxrdf::BlankNode` -Into-> `BlankNodeBuf`
//! - `oxrdf::BlankNodeRef<'a>` -Into-> `&'a BlankNode`
//! - `BlankNodeBuf` -Into-> `oxrdf::BlankNode`
//! - `&BlankNode` -Into-> `oxrdf::BlankNodeRef<'_>`
//! - `oxrdf::Literal` -TryInto-> `Literal`
//! - `&Literal` -Into-> `oxrdf::Literal`
//! - `oxrdf::NamedOrBlankNode` -Into-> `SubjectBuf`
//! - `Subject<'_>` -Into-> `oxrdf::NamedOrBlankNode`
//! - `oxrdf::Term` -TryInto-> `ObjectBuf`
//! - `Object<'_>` -Into-> `oxrdf::Term`
//!
//! Language tags are lowercased when converting to [`oxrdf`], matching the
//! normalization applied by [`oxrdf::Literal::new_language_tagged_literal`].
//! RDF 1.2 triple terms and directional language-tagged strings have no
//! term-dr representation, so those conversions fail.

use snafu::Snafu;

use crate::{
    BlankNode, BlankNodeBuf, LanguageTagBuf, LexicalFormBuf, Literal, Object, ObjectBuf, Subject,
    SubjectBuf,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Snafu)]
pub enum ConversionError {
    #[snafu(display("directional language-tagged strings are not supported"))]
    DirectionalLanguageTaggedString,
    #[snafu(display("triple terms are not supported"))]
    TripleTerm,
}

impl From<oxrdf::BlankNode> for BlankNodeBuf {
    fn from(node: oxrdf::BlankNode) -> Self {
        unsafe { Self::new_unchecked(node.into_string()) }
    }
}

impl<'a> From<oxrdf::BlankNodeRef<'a>> for &'a BlankNode {
    fn from(node: oxrdf::BlankNodeRef<'a>) -> Self {
        unsafe { BlankNode::new_unchecked(node.as_str()) }
    }
}

impl From<BlankNodeBuf> for oxrdf::BlankNode {
    fn from(node: BlankNodeBuf) -> Self {
        Self::new_unchecked(String::from(node))
    }
}

impl<'a> From<&'a BlankNode> for oxrdf::BlankNodeRef<'a> {
    fn from(node: &'a BlankNode) -> Self {
        Self::new_unchecked(node.as_ref())
    }
}

impl TryFrom<oxrdf::Literal> for Literal {
    type Error = ConversionError;

    fn try_from(literal: oxrdf::Literal) -> Result<Self, ConversionError> {
        let (value, datatype, language, direction) = literal.destruct();
        if direction.is_some() {
            return Err(ConversionError::DirectionalLanguageTaggedString);
        }
        let lexical_form = unsafe { LexicalFormBuf::new_unchecked(value) };
        Ok(match (datatype, language) {
            (_, Some(language)) => Literal::LanguageTagged {
                lexical_form,
                language_tag: unsafe { LanguageTagBuf::new_unchecked(language) },
            },
            (Some(datatype), None) => Literal::Typed {
                lexical_form,
                datatype_iri: datatype.into(),
            },
            (None, None) => Literal::Simple { lexical_form },
        })
    }
}

impl From<&Literal> for oxrdf::Literal {
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Simple { lexical_form } => {
                let value: &str = lexical_form;
                Self::new_simple_literal(value)
            }
            Literal::Typed {
                lexical_form,
                datatype_iri,
            } => {
                let value: &str = lexical_form;
                Self::new_typed_literal(value, oxrdf::NamedNodeRef::from(&**datatype_iri))
            }
            Literal::LanguageTagged {
                lexical_form,
                language_tag,
            } => {
                let value: &str = lexical_form;
                let language: &str = language_tag;
                Self::new_language_tagged_literal_unchecked(value, language.to_ascii_lowercase())
            }
        }
    }
}

impl From<oxrdf::NamedOrBlankNode> for SubjectBuf {
    fn from(subject: oxrdf::NamedOrBlankNode) -> Self {
        match subject {
            oxrdf::NamedOrBlankNode::NamedNode(node) => Self::Iri(node.into()),
            oxrdf::NamedOrBlankNode::BlankNode(node) => Self::BlankNode(node.into()),
        }
    }
}

impl From<Subject<'_>> for oxrdf::NamedOrBlankNode {
    fn from(subject: Subject<'_>) -> Self {
        match subject {
            Subject::Iri(iri) => oxrdf::NamedNodeRef::from(iri).into(),
            Subject::BlankNode(node) => oxrdf::BlankNodeRef::from(node).into(),
        }
    }
}

impl TryFrom<oxrdf::Term> for ObjectBuf {
    type Error = ConversionError;

    fn try_from(term: oxrdf::Term) -> Result<Self, ConversionError> {
        Ok(match term {
            oxrdf::Term::NamedNode(node) => Self::Iri(node.into()),
            oxrdf::Term::BlankNode(node) => Self::BlankNode(node.into()),
            oxrdf::Term::Literal(literal) => Self::Literal(literal.try_into()?),
            oxrdf::Term::Triple(_) => return Err(ConversionError::TripleTerm),
        })
    }
}

impl From<Object<'_>> for oxrdf::Term {
    fn from(object: Object<'_>) -> Self {
        match object {
            Object::Iri(iri) => oxrdf::NamedNodeRef::from(iri).into(),
            Object::BlankNode(node) => oxrdf::BlankNodeRef::from(node).into(),
            Object::Literal(literal) => oxrdf::Literal::from(literal).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_node_round_trip() {
        let node = oxrdf::BlankNode::new("b0").unwrap();
        let buf = BlankNodeBuf::from(node.clone());
        assert_eq!(AsRef::<str>::as_ref(&buf), "b0");
        assert_eq!(oxrdf::BlankNode::from(buf), node);
        let borrowed: &BlankNode = node.as_ref().into();
        assert_eq!(oxrdf::BlankNodeRef::from(borrowed), node.as_ref());
    }

    #[test]
    fn simple_literal_round_trip() {
        let literal = oxrdf::Literal::new_simple_literal("hello");
        let converted = Literal::try_from(literal.clone()).unwrap();
        assert!(matches!(converted, Literal::Simple { .. }));
        assert_eq!(oxrdf::Literal::from(&converted), literal);
    }

    #[test]
    fn typed_literal_round_trip() {
        let datatype = oxrdf::NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap();
        let literal = oxrdf::Literal::new_typed_literal("42", datatype);
        let converted = Literal::try_from(literal.clone()).unwrap();
        assert!(matches!(converted, Literal::Typed { .. }));
        assert_eq!(oxrdf::Literal::from(&converted), literal);
    }

    #[test]
    fn language_tagged_literal_round_trip() {
        let literal = oxrdf::Literal::new_language_tagged_literal("hi", "en-US").unwrap();
        let converted = Literal::try_from(literal.clone()).unwrap();
        assert!(matches!(converted, Literal::LanguageTagged { .. }));
        assert_eq!(oxrdf::Literal::from(&converted), literal);
    }

    #[test]
    fn language_tag_is_lowercased() {
        let literal = Literal::LanguageTagged {
            lexical_form: LexicalFormBuf::new("hi").unwrap(),
            language_tag: LanguageTagBuf::new("en-US").unwrap(),
        };
        assert_eq!(oxrdf::Literal::from(&literal).language(), Some("en-us"));
    }

    #[test]
    fn directional_literal_is_rejected() {
        let literal = oxrdf::Literal::new_directional_language_tagged_literal(
            "hi",
            "en",
            oxrdf::BaseDirection::Ltr,
        )
        .unwrap();
        assert_eq!(
            Literal::try_from(literal),
            Err(ConversionError::DirectionalLanguageTaggedString),
        );
    }

    #[test]
    fn subject_round_trip() {
        let subject =
            oxrdf::NamedOrBlankNode::from(oxrdf::NamedNode::new("http://example.com/a").unwrap());
        let buf = SubjectBuf::from(subject.clone());
        assert!(matches!(buf, SubjectBuf::Iri(_)));
        assert_eq!(oxrdf::NamedOrBlankNode::from(Subject::from(&buf)), subject);
    }

    #[test]
    fn triple_term_is_rejected() {
        let triple = oxrdf::Triple::new(
            oxrdf::NamedNode::new("http://example.com/s").unwrap(),
            oxrdf::NamedNode::new("http://example.com/p").unwrap(),
            oxrdf::NamedNode::new("http://example.com/o").unwrap(),
        );
        assert_eq!(
            ObjectBuf::try_from(oxrdf::Term::Triple(Box::new(triple))),
            Err(ConversionError::TripleTerm),
        );
    }
}
