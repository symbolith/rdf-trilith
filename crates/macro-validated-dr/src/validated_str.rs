/// Defines a borrowed, unsized newtype over `str` with validation.
///
/// The generated type is `#[repr(transparent)]` and wraps `str` directly,
/// so `&Name` is a thin pointer that can be created from `&str` via a
/// zero-cost pointer cast after validation.
///
/// # Syntax
///
/// ```ignore
/// validated_str!($vis $Name, $validate => $Error);
/// ```
///
/// - `$vis`      — visibility qualifier (e.g. `pub`, `pub(crate)`, or empty)
/// - `$Name`     — identifier for the generated type
/// - `$validate` — expression of type `fn(&str) -> Result<T, $Error>`; the `Ok` value is discarded
/// - `$Error`    — the error type returned on validation failure
///
/// # Generated API
///
/// - `$Name::new(&str) -> Result<&$Name, $Error>`
/// - `$Name::new_unchecked(&str) -> &$Name` (unsafe)
///
/// # Generated trait impls
///
/// `$Name` is unsized, so values only exist behind references.
///
/// ```text
/// &$Name  -Deref→    &str
/// &$Name  -AsRef→    &str
/// &$Name  -AsRef→    &$Name
/// &$Name  -Borrow→   &str
/// &str    -TryFrom→  Result<&$Name, $Error>   (validates, no alloc)
///
/// (+ Debug, Display, Eq, PartialEq, Hash, Ord, PartialOrd)
/// ```
#[macro_export]
macro_rules! validated_str {
    ($vis:vis $Name:ident, $validate:expr => $Error:ty) => {
        #[repr(transparent)]
        $vis struct $Name(str);

        impl $Name {
            $vis fn new(input: &str) -> ::core::result::Result<&Self, $Error> {
                ($validate)(input)?;
                Ok(unsafe { Self::new_unchecked(input) })
            }

            /// # Safety
            ///
            /// `input` must satisfy this type's validation invariant.
            #[allow(unsafe_code)]
            $vis unsafe fn new_unchecked(input: &str) -> &Self {
                debug_assert!(($validate)(input).is_ok());
                unsafe { &*(input as *const str as *const Self) }
            }
        }

        impl ::core::convert::AsRef<str> for $Name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::core::convert::AsRef<$Name> for $Name {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl ::core::borrow::Borrow<str> for $Name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl ::core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(::core::stringify!($Name))
                    .field(&&self.0)
                    .finish()
            }
        }

        impl ::core::ops::Deref for $Name {
            type Target = str;

            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::core::cmp::Eq for $Name {}

        impl ::core::cmp::PartialEq for $Name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl ::core::hash::Hash for $Name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
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

        impl<'a> ::core::convert::TryFrom<&'a str> for &'a $Name {
            type Error = $Error;

            fn try_from(input: &'a str) -> ::core::result::Result<Self, $Error> {
                $Name::new(input)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

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

    fn hash_of<T: Hash + ?Sized>(val: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn new_valid() {
        let non_empty = NonEmpty::new("hello").unwrap();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
    }

    #[test]
    fn new_invalid() {
        assert_eq!(NonEmpty::new("").unwrap_err(), ValidationError::Empty);
    }

    #[test]
    fn pointer_identity() {
        let input = "hello";
        let non_empty = NonEmpty::new(input).unwrap();
        assert!(std::ptr::eq(
            AsRef::<str>::as_ref(non_empty).as_bytes().as_ptr(),
            input.as_ptr()
        ));
    }

    #[test]
    fn deref() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let derefed: &str = non_empty;
        assert_eq!(derefed, "hello");
    }

    #[test]
    fn borrow_and_asref() {
        let non_empty = NonEmpty::new("hello").unwrap();
        let borrowed: &str = non_empty.borrow();
        let as_ref: &str = non_empty.as_ref();
        assert_eq!(borrowed, "hello");
        assert_eq!(as_ref, "hello");
    }

    #[test]
    fn eq_ord_hash() {
        let first = NonEmpty::new("abc").unwrap();
        let second = NonEmpty::new("abc").unwrap();
        let third = NonEmpty::new("xyz").unwrap();
        assert_eq!(first, second);
        assert_ne!(first, third);
        assert!(first < third);
        assert_eq!(hash_of(first), hash_of(second));
    }

    #[test]
    fn try_from_str() {
        let non_empty: &NonEmpty = "hello".try_into().unwrap();
        assert_eq!(AsRef::<str>::as_ref(non_empty), "hello");
        let result: Result<&NonEmpty, _> = "".try_into();
        assert_eq!(result.unwrap_err(), ValidationError::Empty);
    }

    proptest! {
        #[test]
        fn content_and_pointer(input in "[a-zA-Z]+") {
            let non_empty = NonEmpty::new(&input).unwrap();
            prop_assert_eq!(AsRef::<str>::as_ref(non_empty), input.as_str());
            prop_assert!(std::ptr::eq(AsRef::<str>::as_ref(non_empty).as_bytes().as_ptr(), input.as_ptr()));
        }
    }
}
