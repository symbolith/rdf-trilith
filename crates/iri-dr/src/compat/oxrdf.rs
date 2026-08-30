//! Conversions between [`oxrdf`] IRI types and iri-dr types.
//!
//! - `oxrdf::NamedNode` -Into-> `IriBuf`
//! - `oxrdf::NamedNodeRef<'a>` -Into-> `&'a Iri`
//! - `IriBuf` -Into-> `oxrdf::NamedNode`
//! - `&Iri` -Into-> `oxrdf::NamedNodeRef<'_>`

use crate::{Iri, IriBuf};

impl From<oxrdf::NamedNode> for IriBuf {
    fn from(node: oxrdf::NamedNode) -> Self {
        unsafe { Self::new_unchecked(node.into_string()) }
    }
}

impl<'a> From<oxrdf::NamedNodeRef<'a>> for &'a Iri {
    fn from(node: oxrdf::NamedNodeRef<'a>) -> Self {
        unsafe { Iri::new_unchecked(node.as_str()) }
    }
}

impl From<IriBuf> for oxrdf::NamedNode {
    fn from(iri: IriBuf) -> Self {
        Self::new_unchecked(String::from(iri))
    }
}

impl<'a> From<&'a Iri> for oxrdf::NamedNodeRef<'a> {
    fn from(iri: &'a Iri) -> Self {
        Self::new_unchecked(iri.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_round_trip() {
        let node = oxrdf::NamedNode::new("http://example.com/a").unwrap();
        let iri = IriBuf::from(node.clone());
        assert_eq!(AsRef::<str>::as_ref(&iri), "http://example.com/a");
        assert_eq!(oxrdf::NamedNode::from(iri), node);
    }

    #[test]
    fn borrowed_round_trip() {
        let node = oxrdf::NamedNodeRef::new("http://example.com/a").unwrap();
        let iri: &Iri = node.into();
        assert_eq!(AsRef::<str>::as_ref(iri), "http://example.com/a");
        assert_eq!(oxrdf::NamedNodeRef::from(iri), node);
    }
}
