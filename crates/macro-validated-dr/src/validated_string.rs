/// Defines an owned newtype over `String` paired with a [`validated_str!`] borrowed type.
///
/// The generated type is `#[repr(transparent)]` and wraps `String` directly.
/// It derefs to `&$Borrowed`, giving you the same `str`-like / `$Borrowed`-like
/// API on both the owned and borrowed sides.
///
/// # Syntax
///
/// ```ignore
/// validated_string!($vis $Name for $Borrowed, $validate => $Error);
/// ```
///
/// - `$vis`      — visibility qualifier (e.g. `pub`, `pub(crate)`, or empty)
/// - `$Name`     — identifier for the generated owned type
/// - `$Borrowed` — the corresponding `validated_str!` type
/// - `$validate` — expression of type `fn(&str) -> Result<T, $Error>`; the `Ok` value is discarded
/// - `$Error`    — the error type returned on validation failure
///
/// # Generated API
///
/// - `$Name::new(&str) -> Result<$Name, $Error>`
/// - `$Name::new_unchecked(String) -> $Name` (unsafe)
///
/// # Generated trait impls
///
/// ```text
/// $Name       -Deref→    &$Borrowed
/// $Name       -AsRef→    &$Borrowed
/// $Name       -AsRef→    &str
/// $Name       -Borrow→   &$Borrowed
/// $Name       -Borrow→   &str
/// &$Borrowed  -From→     $Name                    (allocates)
/// $Borrowed   -ToOwned→  $Name                    (allocates)
/// &str        -FromStr→  Result<$Name, $Error>    (validates + allocates)
/// String      -TryFrom→  Result<$Name, $Error>    (validates, no realloc)
/// $Name       -Into→     String                   (unwraps, no alloc)
///
/// (+ Clone, Debug, Display, Eq, PartialEq (also vs $Borrowed, &$Borrowed),
///    Hash, Ord, PartialOrd)
/// ```
#[macro_export]
macro_rules! validated_string {
    ($vis:vis $Name:ident for $Borrowed:ident, $validate:expr => $Error:ty) => {
        #[repr(transparent)]
        #[derive(Clone)]
        $vis struct $Name(String);

        impl $Name {
            $vis fn new(input: &str) -> ::core::result::Result<Self, $Error> {
                ($validate)(input)?;
                Ok(Self(input.to_owned()))
            }

            /// # Safety
            ///
            /// `input` must satisfy this type's validation invariant.
            #[allow(unsafe_code)]
            $vis unsafe fn new_unchecked(input: String) -> Self {
                debug_assert!(($validate)(input.as_str()).is_ok());
                Self(input)
            }
        }

        impl ::core::convert::AsRef<str> for $Name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::core::convert::AsRef<$Borrowed> for $Name {
            fn as_ref(&self) -> &$Borrowed {
                unsafe { $Borrowed::new_unchecked(&self.0) }
            }
        }

        impl ::core::borrow::Borrow<str> for $Name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl ::core::borrow::Borrow<$Borrowed> for $Name {
            fn borrow(&self) -> &$Borrowed {
                unsafe { $Borrowed::new_unchecked(&self.0) }
            }
        }

        impl ::core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(::core::stringify!($Name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::core::ops::Deref for $Name {
            type Target = $Borrowed;

            fn deref(&self) -> &$Borrowed {
                unsafe { $Borrowed::new_unchecked(&self.0) }
            }
        }

        impl ::core::cmp::Eq for $Name {}

        impl ::core::cmp::PartialEq for $Name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl ::core::cmp::PartialEq<$Borrowed> for $Name {
            fn eq(&self, other: &$Borrowed) -> bool {
                *self.0 == other.0
            }
        }

        impl ::core::cmp::PartialEq<&$Borrowed> for $Name {
            fn eq(&self, other: &&$Borrowed) -> bool {
                *self.0 == other.0
            }
        }

        impl ::core::cmp::PartialEq<$Name> for $Borrowed {
            fn eq(&self, other: &$Name) -> bool {
                self.0 == *other.0
            }
        }

        impl ::core::cmp::PartialEq<$Name> for &$Borrowed {
            fn eq(&self, other: &$Name) -> bool {
                self.0 == *other.0
            }
        }

        impl ::core::convert::From<&$Borrowed> for $Name {
            fn from(borrowed: &$Borrowed) -> Self {
                Self(borrowed.0.to_owned())
            }
        }

        impl ::core::str::FromStr for $Name {
            type Err = $Error;

            fn from_str(input: &str) -> ::core::result::Result<Self, $Error> {
                Self::new(input)
            }
        }

        impl ::core::hash::Hash for $Name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl ::core::convert::From<$Name> for String {
            fn from(value: $Name) -> String {
                value.0
            }
        }

        impl ::core::cmp::Ord for $Name {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl ::core::cmp::PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl ::std::borrow::ToOwned for $Borrowed {
            type Owned = $Name;

            fn to_owned(&self) -> $Name {
                $Name(self.0.to_owned())
            }
        }

        impl ::core::convert::TryFrom<String> for $Name {
            type Error = $Error;

            fn try_from(input: String) -> ::core::result::Result<Self, $Error> {
                ($validate)(input.as_str())?;
                Ok(Self(input))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use proptest::prelude::*;

    #[derive(Debug, PartialEq, snafu::Snafu)]
    enum ValidationError {
        #[snafu(display("string is empty"))]
        Empty,
    }

    fn validate_non_empty(input: &str) -> Result<(), ValidationError> {
        if input.is_empty() {
            Err(ValidationError::Empty)
        } else {
            Ok(())
        }
    }

    validated_str!(NonEmpty, validate_non_empty => ValidationError);
    validated_string!(NonEmptyBuf for NonEmpty, validate_non_empty => ValidationError);

    #[test]
    fn new_valid() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let text: &str = buf.as_ref();
        assert_eq!(text, "hello");
    }

    #[test]
    fn new_invalid() {
        assert_eq!(NonEmptyBuf::new("").unwrap_err(), ValidationError::Empty);
    }

    #[test]
    fn new_unchecked_valid() {
        let buf = unsafe { NonEmptyBuf::new_unchecked("hello".to_string()) };
        let text: &str = buf.as_ref();
        assert_eq!(text, "hello");
    }

    #[test]
    fn asref_borrowed() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let non_empty: &NonEmpty = buf.as_ref();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
        let buf_str: &str = buf.as_ref();
        assert!(std::ptr::eq(
            AsRef::<str>::as_ref(non_empty).as_ptr(),
            buf_str.as_ptr()
        ));
    }

    #[test]
    fn borrow_borrowed() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let non_empty: &NonEmpty = buf.borrow();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
        let buf_str: &str = buf.as_ref();
        assert!(std::ptr::eq(
            AsRef::<str>::as_ref(non_empty).as_ptr(),
            buf_str.as_ptr()
        ));
    }

    #[test]
    fn deref() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let non_empty: &NonEmpty = &buf;
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
        let buf_str: &str = buf.as_ref();
        assert!(std::ptr::eq(
            AsRef::<str>::as_ref(non_empty).as_ptr(),
            buf_str.as_ptr()
        ));
    }

    #[test]
    fn three_paths_agree() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let via_asref: &NonEmpty = buf.as_ref();
        let via_borrow: &NonEmpty = buf.borrow();
        let via_deref: &NonEmpty = &buf;
        assert!(std::ptr::eq(via_asref, via_borrow));
        assert!(std::ptr::eq(via_borrow, via_deref));
    }

    #[test]
    fn to_owned_round_trip() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let buf: NonEmptyBuf = non_empty.to_owned();
        let non_empty2: &NonEmpty = &buf;
        let text: &str = non_empty2.as_ref();
        assert_eq!(text, "hello");
    }

    #[test]
    fn from_borrowed() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let buf = NonEmptyBuf::from(non_empty);
        assert_eq!(<NonEmptyBuf as AsRef<str>>::as_ref(&buf), "hello");
    }

    #[test]
    fn try_from_string() {
        let ok = NonEmptyBuf::try_from("hello".to_string()).unwrap();
        assert_eq!(<NonEmptyBuf as AsRef<str>>::as_ref(&ok), "hello");
        assert_eq!(
            NonEmptyBuf::try_from(String::new()).unwrap_err(),
            ValidationError::Empty,
        );
    }

    #[test]
    fn from_str() {
        let buf: NonEmptyBuf = "hello".parse().unwrap();
        assert_eq!(<NonEmptyBuf as AsRef<str>>::as_ref(&buf), "hello");
    }

    #[test]
    fn into_string() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let text: String = buf.into();
        assert_eq!(text, "hello");
    }

    #[test]
    fn clone() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let cloned = buf.clone();
        assert_eq!(
            <NonEmptyBuf as AsRef<str>>::as_ref(&buf),
            <NonEmptyBuf as AsRef<str>>::as_ref(&cloned),
        );
        let ptr_a: *const str = <NonEmptyBuf as AsRef<str>>::as_ref(&buf);
        let ptr_b: *const str = <NonEmptyBuf as AsRef<str>>::as_ref(&cloned);
        assert!(!std::ptr::eq(ptr_a, ptr_b));
    }

    #[test]
    fn eq_cross_type() {
        let buf = NonEmptyBuf::new("hello").unwrap();
        let non_empty = NonEmpty::new("hello").unwrap();
        assert_eq!(buf, *non_empty);
        assert_eq!(*non_empty, buf);
        assert_eq!(buf, non_empty);
        assert_eq!(non_empty, buf);
    }

    #[test]
    fn full_round_trip() {
        let input = "hello";
        let non_empty = NonEmpty::new(input).unwrap();
        let buf: NonEmptyBuf = non_empty.to_owned();
        let non_empty2: &NonEmpty = &buf;
        let back: &str = non_empty2.as_ref();
        assert_eq!(back, input);
    }

    proptest! {
        #[test]
        fn deref_round_trip(input in "[a-zA-Z]+") {
            let buf = NonEmptyBuf::new(&input).unwrap();
            let non_empty: &NonEmpty = &buf;
            prop_assert_eq!(AsRef::<str>::as_ref(non_empty), input.as_str());
        }
    }
}
