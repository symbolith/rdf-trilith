use rdf_trilith_iri::IriAbsolute;

use crate::link::{ConversionError, Link, Target};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Autolink<'a>(pub &'a IriAbsolute);

impl<'a> From<Autolink<'a>> for Link<'a> {
    fn from(link: Autolink<'a>) -> Self {
        Link {
            target: Target::Absolute(link.0),
            text: None,
            title: None,
        }
    }
}

impl<'a> TryFrom<Link<'a>> for Autolink<'a> {
    type Error = ConversionError;

    fn try_from(link: Link<'a>) -> Result<Self, ConversionError> {
        if link.text.is_some() {
            return Err(ConversionError::UnsupportedText);
        }
        if link.title.is_some() {
            return Err(ConversionError::UnsupportedTitle);
        }
        let Target::Absolute(iri) = link.target else {
            return Err(ConversionError::UnsupportedTarget);
        };
        Ok(Autolink(iri))
    }
}
