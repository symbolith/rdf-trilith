/// Declares a morphism relationship between two [`validated_str!`] / [`validated_string!`] pairs.
///
/// The generated impls use zero-cost pointer casts so that a reference to
/// the narrow (more restrictive) type also serves as a reference to the
/// broad (less restrictive) type without copying or re-validating.
///
/// The invoker asserts that every string accepted by the narrow validator
/// is also accepted by the broad validator; the widening impls rely on it.
///
/// # Syntax
///
/// ```ignore
/// morphism!(NarrowBuf for Narrow => BroadBuf for Broad);
/// ```
///
/// - `Narrow` — the more restrictive borrowed type (e.g. `Alpha`)
/// - `Broad`  — the less restrictive borrowed type (e.g. `NonEmpty`)
///
/// # Generated trait impls
///
/// - `AsRef<Broad> for Narrow`
/// - `AsRef<Broad> for NarrowBuf`
/// - `From<NarrowBuf> for BroadBuf`
/// - `TryFrom<&Broad> for &Narrow`
/// - `TryFrom<BroadBuf> for NarrowBuf`
#[macro_export]
macro_rules! morphism {
    ($NarrowBuf:ident for $Narrow:ident => $BroadBuf:ident for $Broad:ident) => {
        impl ::core::convert::AsRef<$Broad> for $Narrow {
            fn as_ref(&self) -> &$Broad {
                unsafe { $Broad::new_unchecked(::core::convert::AsRef::<str>::as_ref(self)) }
            }
        }

        impl ::core::convert::AsRef<$Broad> for $NarrowBuf {
            fn as_ref(&self) -> &$Broad {
                unsafe { $Broad::new_unchecked(::core::convert::AsRef::<str>::as_ref(self)) }
            }
        }

        impl ::core::convert::From<$NarrowBuf> for $BroadBuf {
            fn from(value: $NarrowBuf) -> Self {
                let raw: String = value.into();
                unsafe { Self::new_unchecked(raw) }
            }
        }

        impl<'a> ::core::convert::TryFrom<&'a $Broad> for &'a $Narrow {
            type Error = <&'a $Narrow as ::core::convert::TryFrom<&'a str>>::Error;

            fn try_from(value: &'a $Broad) -> ::core::result::Result<Self, Self::Error> {
                $Narrow::new(::core::convert::AsRef::<str>::as_ref(value))
            }
        }

        impl ::core::convert::TryFrom<$BroadBuf> for $NarrowBuf {
            type Error = <$NarrowBuf as ::core::convert::TryFrom<String>>::Error;

            fn try_from(value: $BroadBuf) -> ::core::result::Result<Self, Self::Error> {
                let raw: String = value.into();
                Self::try_from(raw)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{validated_str, validated_string};

    #[derive(Debug, PartialEq, snafu::Snafu)]
    enum ValidationError {
        #[snafu(display("string is empty"))]
        Empty,
        #[snafu(display("string contains non-alpha character"))]
        NotAlpha,
    }

    fn validate_non_empty(input: &str) -> Result<(), ValidationError> {
        if input.is_empty() {
            Err(ValidationError::Empty)
        } else {
            Ok(())
        }
    }

    fn validate_alpha(input: &str) -> Result<(), ValidationError> {
        if input.is_empty() {
            return Err(ValidationError::Empty);
        }
        if !input.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Err(ValidationError::NotAlpha);
        }
        Ok(())
    }

    validated_str!(NonEmpty, validate_non_empty => ValidationError);
    validated_string!(NonEmptyBuf for NonEmpty, validate_non_empty => ValidationError);

    validated_str!(Alpha, validate_alpha => ValidationError);
    validated_string!(AlphaBuf for Alpha, validate_alpha => ValidationError);

    morphism!(AlphaBuf for Alpha => NonEmptyBuf for NonEmpty);

    #[test]
    fn borrowed_asref_content() {
        let alpha = Alpha::new("hello").unwrap();
        let non_empty: &NonEmpty = alpha.as_ref();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
    }

    #[test]
    fn borrowed_asref_pointer() {
        let alpha = Alpha::new("hello").unwrap();
        let non_empty: &NonEmpty = alpha.as_ref();
        assert!(std::ptr::eq(
            <Alpha as AsRef<str>>::as_ref(alpha).as_ptr(),
            <NonEmpty as AsRef<str>>::as_ref(non_empty).as_ptr(),
        ));
    }

    #[test]
    fn owned_asref_content_and_pointer() {
        let alpha_buf = AlphaBuf::new("hello").unwrap();
        let non_empty: &NonEmpty = alpha_buf.as_ref();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
        let buf_str: &str = alpha_buf.as_ref();
        assert!(std::ptr::eq(
            <NonEmpty as AsRef<str>>::as_ref(non_empty).as_ptr(),
            buf_str.as_ptr(),
        ));
    }

    #[test]
    fn owned_from() {
        let alpha_buf = AlphaBuf::new("hello").unwrap();
        let non_empty_buf: NonEmptyBuf = alpha_buf.into();
        assert_eq!(<NonEmptyBuf as AsRef<str>>::as_ref(&non_empty_buf), "hello");
    }

    #[test]
    fn round_trip() {
        let input = "hello";
        let alpha = Alpha::new(input).unwrap();
        let non_empty: &NonEmpty = alpha.as_ref();
        let back: &str = non_empty.as_ref();
        assert_eq!(back, input);

        let alpha_buf = AlphaBuf::new(input).unwrap();
        let non_empty_ref: &NonEmpty = alpha_buf.as_ref();
        let back2: &str = non_empty_ref.as_ref();
        assert_eq!(back2, input);
        let non_empty_buf: NonEmptyBuf = AlphaBuf::new(input).unwrap().into();
        let non_empty3: &NonEmpty = &non_empty_buf;
        assert_eq!(AsRef::<str>::as_ref(non_empty3), input);
    }

    #[test]
    fn full_round_trip_with_morphism() {
        let input = "hello";
        let alpha = Alpha::new(input).unwrap();
        let alpha_buf: AlphaBuf = alpha.to_owned();
        let alpha2: &Alpha = &alpha_buf;
        let non_empty: &NonEmpty = alpha2.as_ref();
        let back: &str = non_empty.as_ref();
        assert_eq!(back, input);
    }

    #[test]
    fn borrowed_try_from_valid() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let alpha: &Alpha = non_empty.try_into().unwrap();
        assert_eq!(<Alpha as AsRef<str>>::as_ref(alpha), "hello");
    }

    #[test]
    fn borrowed_try_from_invalid() {
        let non_empty = NonEmpty::new("123").unwrap();
        let result: Result<&Alpha, _> = non_empty.try_into();
        assert_eq!(result.unwrap_err(), ValidationError::NotAlpha);
    }

    #[test]
    fn borrowed_try_from_pointer_identity() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let alpha: &Alpha = non_empty.try_into().unwrap();
        assert!(std::ptr::eq(
            <NonEmpty as AsRef<str>>::as_ref(non_empty).as_ptr(),
            <Alpha as AsRef<str>>::as_ref(alpha).as_ptr(),
        ));
    }

    #[test]
    fn owned_try_from_valid() {
        let non_empty_buf = NonEmptyBuf::new("hello").unwrap();
        let alpha_buf: AlphaBuf = non_empty_buf.try_into().unwrap();
        assert_eq!(<AlphaBuf as AsRef<str>>::as_ref(&alpha_buf), "hello");
    }

    #[test]
    fn owned_try_from_invalid() {
        let non_empty_buf = NonEmptyBuf::new("123").unwrap();
        let result: Result<AlphaBuf, _> = non_empty_buf.try_into();
        assert_eq!(result.unwrap_err(), ValidationError::NotAlpha);
    }

    #[test]
    fn narrowing_round_trip() {
        let input = "hello";
        let alpha = Alpha::new(input).unwrap();
        let non_empty: &NonEmpty = alpha.as_ref();
        let back: &Alpha = non_empty.try_into().unwrap();
        assert_eq!(<Alpha as AsRef<str>>::as_ref(back), input);

        let alpha_buf = AlphaBuf::new(input).unwrap();
        let non_empty_buf: NonEmptyBuf = alpha_buf.into();
        let back_buf: AlphaBuf = non_empty_buf.try_into().unwrap();
        assert_eq!(<AlphaBuf as AsRef<str>>::as_ref(&back_buf), input);
    }

    proptest! {
        #[test]
        fn borrowed_coercion(input in "[a-zA-Z]+") {
            let alpha = Alpha::new(&input).unwrap();
            let non_empty: &NonEmpty = alpha.as_ref();
            prop_assert_eq!(<NonEmpty as AsRef<str>>::as_ref(non_empty), input.as_str());
            prop_assert!(std::ptr::eq(
                <Alpha as AsRef<str>>::as_ref(alpha).as_ptr(),
                <NonEmpty as AsRef<str>>::as_ref(non_empty).as_ptr(),
            ));
        }

        #[test]
        fn prop_owned_from(input in "[a-zA-Z]+") {
            let alpha_buf = AlphaBuf::new(&input).unwrap();
            let non_empty_buf: NonEmptyBuf = alpha_buf.into();
            prop_assert_eq!(<NonEmptyBuf as AsRef<str>>::as_ref(&non_empty_buf), input.as_str());
        }

        #[test]
        fn full_round_trip(input in "[a-zA-Z]+") {
            let alpha = Alpha::new(&input).unwrap();
            let alpha_buf: AlphaBuf = alpha.to_owned();
            let alpha2: &Alpha = &alpha_buf;
            let non_empty: &NonEmpty = alpha2.as_ref();
            let back: &str = non_empty.as_ref();
            prop_assert_eq!(back, input.as_str());
        }

        #[test]
        fn prop_borrowed_try_from(input in "[a-zA-Z]+") {
            let non_empty = NonEmpty::new(&input).unwrap();
            let alpha: &Alpha = non_empty.try_into().unwrap();
            prop_assert_eq!(<Alpha as AsRef<str>>::as_ref(alpha), input.as_str());
            prop_assert!(std::ptr::eq(
                <NonEmpty as AsRef<str>>::as_ref(non_empty).as_ptr(),
                <Alpha as AsRef<str>>::as_ref(alpha).as_ptr(),
            ));
        }

        #[test]
        fn prop_owned_try_from(input in "[a-zA-Z]+") {
            let non_empty_buf = NonEmptyBuf::new(&input).unwrap();
            let alpha_buf: AlphaBuf = non_empty_buf.try_into().unwrap();
            prop_assert_eq!(<AlphaBuf as AsRef<str>>::as_ref(&alpha_buf), input.as_str());
        }
    }
}
