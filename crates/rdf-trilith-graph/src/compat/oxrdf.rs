//! Conversions between [`oxrdf`] triples and rdf-trilith-graph types.
//!
//! - `oxrdf::Triple` -TryInto-> `TripleBuf`
//! - `Triple<'_>` -Into-> `oxrdf::Triple`
//! - `&TripleBuf` -Into-> `oxrdf::Triple`

use rdf_trilith_term::ConversionError;

use crate::{Triple, TripleBuf};

impl TryFrom<oxrdf::Triple> for TripleBuf {
    type Error = ConversionError;

    fn try_from(triple: oxrdf::Triple) -> Result<Self, ConversionError> {
        Ok(Self {
            subject: triple.subject.into(),
            predicate: triple.predicate.into(),
            object: triple.object.try_into()?,
        })
    }
}

impl From<Triple<'_>> for oxrdf::Triple {
    fn from(triple: Triple<'_>) -> Self {
        Self::new(
            oxrdf::NamedOrBlankNode::from(triple.subject),
            oxrdf::NamedNodeRef::from(triple.predicate),
            oxrdf::Term::from(triple.object),
        )
    }
}

impl From<&TripleBuf> for oxrdf::Triple {
    fn from(triple: &TripleBuf) -> Self {
        Triple::from(triple).into()
    }
}

#[cfg(test)]
mod tests {
    use rdf_trilith_term::{Iri, IriBuf, LexicalFormBuf, Literal};

    use super::*;

    fn iri(text: &str) -> IriBuf {
        IriBuf::new(text).unwrap()
    }

    #[test]
    fn owned_round_trip() {
        let triple = TripleBuf::new(
            iri("http://x/s"),
            iri("http://x/p"),
            Literal::Simple {
                lexical_form: LexicalFormBuf::new("hello").unwrap(),
            },
        );
        let converted = oxrdf::Triple::from(&triple);
        assert_eq!(converted.to_string(), "<http://x/s> <http://x/p> \"hello\"");
        assert_eq!(TripleBuf::try_from(converted), Ok(triple));
    }

    #[test]
    fn borrowed_converts() {
        let triple = Triple::new(
            Iri::new("http://x/s").unwrap(),
            Iri::new("http://x/p").unwrap(),
            Iri::new("http://x/o").unwrap(),
        );
        assert_eq!(
            oxrdf::Triple::from(triple).to_string(),
            "<http://x/s> <http://x/p> <http://x/o>",
        );
    }
}
