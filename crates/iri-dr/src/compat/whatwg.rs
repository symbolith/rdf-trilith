//! Conversions between [`url::Url`] (url.spec.whatwg.org) and iri-dr types.
//!
//! The canonical forms of WHATWG and RFC 3986 overlap without containment,
//! so both directions are fallible.
//!
//! - `&url::Url` -TryInto-> `&Uri`
//! - `&Iri` -TryInto-> `url::Url`

use crate::{Iri, Uri, ValidationError};

impl<'a> TryFrom<&'a url::Url> for &'a Uri {
    type Error = ValidationError;

    fn try_from(url: &'a url::Url) -> Result<Self, ValidationError> {
        Uri::new(url.as_str())
    }
}

impl TryFrom<&Iri> for url::Url {
    type Error = url::ParseError;

    fn try_from(iri: &Iri) -> Result<Self, url::ParseError> {
        url::Url::parse(iri.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text<T: AsRef<str> + ?Sized>(value: &T) -> &str {
        value.as_ref()
    }

    #[test]
    fn url_to_uri() {
        let url = url::Url::parse("http://example.com/a?q#f").unwrap();
        let uri: &Uri = (&url).try_into().unwrap();
        assert_eq!(text(uri), "http://example.com/a?q#f");
        assert!(std::ptr::eq(url.as_str().as_ptr(), text(uri).as_ptr()));
    }

    #[test]
    fn iri_to_url() {
        let iri = Iri::new("HTTP://例え.jp/パス").unwrap();
        let url = url::Url::try_from(iri).unwrap();
        assert_eq!(url.as_str(), "http://xn--r8jz45g.jp/%E3%83%91%E3%82%B9");
    }

    #[test]
    fn iri_to_url_rejected() {
        let iri = Iri::new("http://example.com:99999/").unwrap();
        assert!(url::Url::try_from(iri).is_err());
    }
}
