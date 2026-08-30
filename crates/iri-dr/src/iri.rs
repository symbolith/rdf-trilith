use macro_validated_dr::{morphism, validated_str, validated_string};

use crate::validate::{
    ValidationError, validate_authority, validate_fragment, validate_iri, validate_iri_absolute,
    validate_iri_normalized, validate_iri_reference, validate_iri_relative, validate_path,
    validate_query, validate_scheme, validate_uri, validate_uri_absolute, validate_uri_normalized,
    validate_uri_reference, validate_uri_relative,
};

validated_str!(pub Iri, validate_iri => ValidationError);
validated_str!(pub IriAbsolute, validate_iri_absolute => ValidationError);
validated_str!(pub IriNormalized, validate_iri_normalized => ValidationError);
validated_str!(pub IriReference, validate_iri_reference => ValidationError);
validated_str!(pub IriRelative, validate_iri_relative => ValidationError);
validated_str!(pub Uri, validate_uri => ValidationError);
validated_str!(pub UriAbsolute, validate_uri_absolute => ValidationError);
validated_str!(pub UriNormalized, validate_uri_normalized => ValidationError);
validated_str!(pub UriReference, validate_uri_reference => ValidationError);
validated_str!(pub UriRelative, validate_uri_relative => ValidationError);
validated_string!(pub IriAbsoluteBuf for IriAbsolute, validate_iri_absolute => ValidationError);
validated_string!(pub IriBuf for Iri, validate_iri => ValidationError);
validated_string!(pub IriNormalizedBuf for IriNormalized, validate_iri_normalized => ValidationError);
validated_string!(pub IriReferenceBuf for IriReference, validate_iri_reference => ValidationError);
validated_string!(pub IriRelativeBuf for IriRelative, validate_iri_relative => ValidationError);
validated_string!(pub UriAbsoluteBuf for UriAbsolute, validate_uri_absolute => ValidationError);
validated_string!(pub UriBuf for Uri, validate_uri => ValidationError);
validated_string!(pub UriNormalizedBuf for UriNormalized, validate_uri_normalized => ValidationError);
validated_string!(pub UriReferenceBuf for UriReference, validate_uri_reference => ValidationError);
validated_string!(pub UriRelativeBuf for UriRelative, validate_uri_relative => ValidationError);

pub type Base = IriAbsolute;
pub type BaseBuf = IriAbsoluteBuf;
pub type UriBase = UriAbsolute;
pub type UriBaseBuf = UriAbsoluteBuf;

validated_str!(pub Scheme, validate_scheme => ValidationError);
validated_str!(pub Authority, validate_authority => ValidationError);
validated_str!(pub Path, validate_path => ValidationError);
validated_str!(pub Query, validate_query => ValidationError);
validated_str!(pub Fragment, validate_fragment => ValidationError);

morphism!(IriBuf for Iri => IriReferenceBuf for IriReference);
morphism!(IriAbsoluteBuf for IriAbsolute => IriBuf for Iri);
morphism!(IriAbsoluteBuf for IriAbsolute => IriReferenceBuf for IriReference);
morphism!(IriNormalizedBuf for IriNormalized => IriBuf for Iri);
morphism!(IriNormalizedBuf for IriNormalized => IriReferenceBuf for IriReference);
morphism!(IriRelativeBuf for IriRelative => IriReferenceBuf for IriReference);
morphism!(UriBuf for Uri => IriBuf for Iri);
morphism!(UriBuf for Uri => IriReferenceBuf for IriReference);
morphism!(UriBuf for Uri => UriReferenceBuf for UriReference);
morphism!(UriAbsoluteBuf for UriAbsolute => IriBuf for Iri);
morphism!(UriAbsoluteBuf for UriAbsolute => IriAbsoluteBuf for IriAbsolute);
morphism!(UriAbsoluteBuf for UriAbsolute => IriReferenceBuf for IriReference);
morphism!(UriAbsoluteBuf for UriAbsolute => UriBuf for Uri);
morphism!(UriAbsoluteBuf for UriAbsolute => UriReferenceBuf for UriReference);
morphism!(UriNormalizedBuf for UriNormalized => IriBuf for Iri);
morphism!(UriNormalizedBuf for UriNormalized => IriNormalizedBuf for IriNormalized);
morphism!(UriNormalizedBuf for UriNormalized => IriReferenceBuf for IriReference);
morphism!(UriNormalizedBuf for UriNormalized => UriBuf for Uri);
morphism!(UriNormalizedBuf for UriNormalized => UriReferenceBuf for UriReference);
morphism!(UriReferenceBuf for UriReference => IriReferenceBuf for IriReference);
morphism!(UriRelativeBuf for UriRelative => IriReferenceBuf for IriReference);
morphism!(UriRelativeBuf for UriRelative => IriRelativeBuf for IriRelative);
morphism!(UriRelativeBuf for UriRelative => UriReferenceBuf for UriReference);

#[cfg(test)]
mod tests {
    use super::*;

    fn text<T: AsRef<str> + ?Sized>(value: &T) -> &str {
        value.as_ref()
    }

    #[test]
    fn full_iri_with_unicode() {
        assert!(Iri::new("https://例え.jp/パス?q=素#断片").is_ok());
        assert_eq!(
            Uri::new("https://例え.jp/").unwrap_err(),
            ValidationError::NotAscii
        );
    }

    #[test]
    fn scheme_policies() {
        assert_eq!(
            Iri::new("no-scheme/path").unwrap_err(),
            ValidationError::MissingScheme
        );
        assert_eq!(
            IriRelative::new("http://x").unwrap_err(),
            ValidationError::ForbiddenScheme
        );
        assert!(IriReference::new("//host/path?q#f").is_ok());
        assert!(IriReference::new("").is_ok());
    }

    #[test]
    fn fragment_policies() {
        assert!(Iri::new("http://x/#f").is_ok());
        assert_eq!(
            IriAbsolute::new("http://x/#f").unwrap_err(),
            ValidationError::ForbiddenFragment,
        );
    }

    #[test]
    fn invalid_characters() {
        assert_eq!(
            Iri::new("http://x/a b").unwrap_err(),
            ValidationError::InvalidPath
        );
        assert_eq!(
            Iri::new("http://ex ample/").unwrap_err(),
            ValidationError::InvalidAuthority
        );
        assert_eq!(
            Iri::new("http://x/%GG").unwrap_err(),
            ValidationError::InvalidPath
        );
        assert!(Iri::new("http://x/%C3%A9").is_ok());
    }

    #[test]
    fn components() {
        assert!(Scheme::new("http").is_ok());
        assert_eq!(
            Scheme::new("1http").unwrap_err(),
            ValidationError::InvalidScheme
        );
        assert!(Authority::new("user@example.com:8042").is_ok());
        assert!(Path::new("/over/there").is_ok());
        assert!(Query::new("name=ferret").is_ok());
        assert!(Fragment::new("nose").is_ok());
    }

    #[test]
    fn widening_chain() {
        let uri = Uri::new("http://example.com/a").unwrap();
        let iri: &Iri = uri.as_ref();
        let reference: &IriReference = iri.as_ref();
        assert_eq!(text(reference), "http://example.com/a");
        assert!(std::ptr::eq(text(uri).as_ptr(), text(reference).as_ptr()));
    }

    #[test]
    fn narrowing() {
        let reference = IriReference::new("http://example.com/a").unwrap();
        let iri: &Iri = reference.try_into().unwrap();
        assert_eq!(text(iri), "http://example.com/a");
        let relative = IriReference::new("/only/path").unwrap();
        assert_eq!(
            <&Iri>::try_from(relative).unwrap_err(),
            ValidationError::MissingScheme,
        );
    }

    #[test]
    fn owned_widening() {
        let absolute = IriAbsoluteBuf::new("http://example.com/a").unwrap();
        let reference: IriReferenceBuf = absolute.into();
        assert_eq!(text(&reference), "http://example.com/a");
    }

    #[test]
    fn owned_round_trip() {
        let buf: UriBuf = "http://example.com/a".parse().unwrap();
        let iri_buf: IriBuf = buf.into();
        let narrowed: UriBuf = iri_buf.try_into().unwrap();
        assert_eq!(text(&narrowed), "http://example.com/a");
    }

    #[test]
    fn normalized_accepts_normal_form() {
        assert!(IriNormalized::new("http://example.com/a?q#f").is_ok());
        assert!(IriNormalized::new("http://example.com/PATH").is_ok());
        assert!(IriNormalized::new("http://例え.jp/パス").is_ok());
        assert!(UriNormalized::new("http://example.com/%C3%A9").is_ok());
    }

    #[test]
    fn normalized_case_policies() {
        assert_eq!(
            IriNormalized::new("HTTP://example.com/").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert_eq!(
            IriNormalized::new("http://EXAMPLE.com/").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert!(IriNormalized::new("http://User@example.com/").is_ok());
    }

    #[test]
    fn normalized_percent_policies() {
        assert_eq!(
            IriNormalized::new("http://x/%c3%a9").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert_eq!(
            IriNormalized::new("http://x/%41").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert!(IriNormalized::new("http://x/%3A").is_ok());
    }

    #[test]
    fn normalized_dot_segment_policies() {
        assert_eq!(
            IriNormalized::new("http://x/a/./b").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert_eq!(
            IriNormalized::new("http://x/../a").unwrap_err(),
            ValidationError::NotNormalized
        );
        assert!(IriNormalized::new("http://x/a..b/.hidden").is_ok());
    }

    #[test]
    fn normalized_widening_chain() {
        let uri = UriNormalized::new("http://example.com/a").unwrap();
        let iri: &IriNormalized = uri.as_ref();
        let plain: &Iri = iri.as_ref();
        let reference: &IriReference = plain.as_ref();
        assert_eq!(text(reference), "http://example.com/a");
        assert!(std::ptr::eq(text(uri).as_ptr(), text(reference).as_ptr()));
    }

    #[test]
    fn normalized_narrowing() {
        let iri = Iri::new("http://example.com/a").unwrap();
        let normalized: &IriNormalized = iri.try_into().unwrap();
        assert_eq!(text(normalized), "http://example.com/a");
        let uppercase = Iri::new("HTTP://example.com/").unwrap();
        assert_eq!(
            <&IriNormalized>::try_from(uppercase).unwrap_err(),
            ValidationError::NotNormalized,
        );
    }

    #[test]
    fn resolve_reference_receiver() {
        let base = IriAbsolute::new("http://a/b/c/d;p?q").unwrap();
        assert_eq!(
            text(&IriRelative::new("../g").unwrap().resolve(base)),
            "http://a/b/g"
        );
        assert_eq!(
            text(&IriReference::new("g:h").unwrap().resolve(base)),
            "g:h"
        );
    }

    #[test]
    fn resolve_base_receiver() {
        let base = IriAbsolute::new("http://a/b/c/d;p?q").unwrap();
        assert_eq!(
            text(&base.resolve(IriRelative::new("./g").unwrap())),
            "http://a/b/c/g"
        );
    }

    #[test]
    fn resolve_buf_receivers() {
        let base = IriAbsoluteBuf::new("http://a/b/").unwrap();
        let relative: IriRelativeBuf = "g".parse().unwrap();
        assert_eq!(text(&relative.resolve(&base)), "http://a/b/g");
        assert_eq!(text(&base.resolve(&relative)), "http://a/b/g");
    }

    #[test]
    fn resolve_base_narrowing() {
        let fragmented = Iri::new("http://a/b#f").unwrap();
        assert_eq!(
            <&IriAbsolute>::try_from(fragmented).unwrap_err(),
            ValidationError::ForbiddenFragment,
        );
        let base: &IriAbsolute = Iri::new("http://a/b").unwrap().try_into().unwrap();
        assert_eq!(
            text(&IriRelative::new("c").unwrap().resolve(base)),
            "http://a/c"
        );
    }

    #[test]
    fn resolve_uri_family() {
        let base = UriAbsolute::new("http://a/b/c/d;p?q").unwrap();
        let resolved: UriBuf = UriRelative::new("?y").unwrap().resolve(base);
        assert_eq!(text(&resolved), "http://a/b/c/d;p?y");
    }

    #[test]
    fn resolve_unicode() {
        let base = IriAbsolute::new("http://例え.jp/パス/").unwrap();
        assert_eq!(
            text(&IriRelative::new("素").unwrap().resolve(base)),
            "http://例え.jp/パス/素"
        );
    }

    #[test]
    fn normalized_owned_widening() {
        let normalized = IriNormalizedBuf::new("http://example.com/a").unwrap();
        let iri: IriBuf = normalized.into();
        assert_eq!(text(&iri), "http://example.com/a");
    }
}
