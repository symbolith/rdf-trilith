//! Conversions between [`oxiri`] types and iri-dr types.
//!
//! # `oxiri::Iri` and `Iri` / `IriBuf`
//!
//! - `oxiri::Iri<&str>` -Into-> `&Iri`
//! - `oxiri::Iri<String>` -Into-> `IriBuf`
//! - `oxiri::Iri<String>` -AsRef-> `&Iri`
//! - `oxiri::Iri<&str>` -AsRef-> `&Iri`
//! - `&Iri` -Into-> `oxiri::Iri<&str>`
//! - `IriBuf` -Into-> `oxiri::Iri<String>`
//!
//! # `oxiri::IriRef` and `IriReference` / `IriReferenceBuf`
//!
//! - `oxiri::IriRef<&str>` -Into-> `&IriReference`
//! - `oxiri::IriRef<String>` -Into-> `IriReferenceBuf`
//! - `oxiri::IriRef<String>` -AsRef-> `&IriReference`
//! - `oxiri::IriRef<&str>` -AsRef-> `&IriReference`
//! - `&IriReference` -Into-> `oxiri::IriRef<&str>`
//! - `IriReferenceBuf` -Into-> `oxiri::IriRef<String>`

use crate::{Iri, IriBuf, IriReference, IriReferenceBuf};

impl<'a> From<oxiri::Iri<&'a str>> for &'a Iri {
    fn from(iri: oxiri::Iri<&'a str>) -> Self {
        unsafe { Iri::new_unchecked(iri.into_inner()) }
    }
}

impl From<oxiri::Iri<String>> for IriBuf {
    fn from(iri: oxiri::Iri<String>) -> Self {
        unsafe { Self::new_unchecked(iri.into_inner()) }
    }
}

impl From<IriBuf> for oxiri::Iri<String> {
    fn from(iri: IriBuf) -> Self {
        Self::parse_unchecked(iri.into())
    }
}

impl<'a> From<&'a Iri> for oxiri::Iri<&'a str> {
    fn from(iri: &'a Iri) -> Self {
        Self::parse_unchecked(iri.as_ref())
    }
}

impl AsRef<Iri> for oxiri::Iri<String> {
    fn as_ref(&self) -> &Iri {
        unsafe { Iri::new_unchecked(self.as_str()) }
    }
}

impl AsRef<Iri> for oxiri::Iri<&str> {
    fn as_ref(&self) -> &Iri {
        unsafe { Iri::new_unchecked(self.as_str()) }
    }
}

impl<'a> From<oxiri::IriRef<&'a str>> for &'a IriReference {
    fn from(reference: oxiri::IriRef<&'a str>) -> Self {
        unsafe { IriReference::new_unchecked(reference.into_inner()) }
    }
}

impl From<oxiri::IriRef<String>> for IriReferenceBuf {
    fn from(reference: oxiri::IriRef<String>) -> Self {
        unsafe { Self::new_unchecked(reference.into_inner()) }
    }
}

impl From<IriReferenceBuf> for oxiri::IriRef<String> {
    fn from(reference: IriReferenceBuf) -> Self {
        Self::parse_unchecked(reference.into())
    }
}

impl<'a> From<&'a IriReference> for oxiri::IriRef<&'a str> {
    fn from(reference: &'a IriReference) -> Self {
        Self::parse_unchecked(reference.as_ref())
    }
}

impl AsRef<IriReference> for oxiri::IriRef<String> {
    fn as_ref(&self) -> &IriReference {
        unsafe { IriReference::new_unchecked(self.as_str()) }
    }
}

impl AsRef<IriReference> for oxiri::IriRef<&str> {
    fn as_ref(&self) -> &IriReference {
        unsafe { IriReference::new_unchecked(self.as_str()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text<T: AsRef<str> + ?Sized>(value: &T) -> &str {
        value.as_ref()
    }

    #[test]
    fn iri_round_trip() {
        let source = oxiri::Iri::parse("http://例え.jp/a?q#f".to_owned()).unwrap();
        let buf: IriBuf = source.into();
        assert_eq!(text(&buf), "http://例え.jp/a?q#f");
        let back: oxiri::Iri<String> = buf.into();
        assert_eq!(back.as_str(), "http://例え.jp/a?q#f");
    }

    #[test]
    fn iri_borrowed() {
        let source = oxiri::Iri::parse("http://example.com/a").unwrap();
        let iri: &Iri = source.into();
        assert!(std::ptr::eq(text(iri).as_ptr(), source.as_str().as_ptr()));
        let back: oxiri::Iri<&str> = iri.into();
        assert_eq!(back.as_str(), "http://example.com/a");
    }

    #[test]
    fn iri_as_ref() {
        let owned = oxiri::Iri::parse("http://example.com/a".to_owned()).unwrap();
        let from_owned: &Iri = AsRef::<Iri>::as_ref(&owned);
        assert_eq!(text(from_owned), "http://example.com/a");
        let borrowed = oxiri::Iri::parse("http://example.com/b").unwrap();
        let from_borrowed: &Iri = AsRef::<Iri>::as_ref(&borrowed);
        assert_eq!(text(from_borrowed), "http://example.com/b");
    }

    #[test]
    fn reference_round_trip() {
        let source = oxiri::IriRef::parse("../g?q".to_owned()).unwrap();
        let buf: IriReferenceBuf = source.into();
        assert_eq!(text(&buf), "../g?q");
        let back: oxiri::IriRef<String> = buf.into();
        assert_eq!(back.as_str(), "../g?q");
    }

    #[test]
    fn reference_borrowed() {
        let source = oxiri::IriRef::parse("//host/path").unwrap();
        let reference: &IriReference = source.into();
        assert_eq!(text(reference), "//host/path");
        let back: oxiri::IriRef<&str> = reference.into();
        assert_eq!(back.as_str(), "//host/path");
    }

    #[test]
    fn reference_as_ref() {
        let owned = oxiri::IriRef::parse("../g".to_owned()).unwrap();
        let reference: &IriReference = AsRef::<IriReference>::as_ref(&owned);
        assert_eq!(text(reference), "../g");
    }
}
