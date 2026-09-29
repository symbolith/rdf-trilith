/// Generates the conversion surface between term types and enums.
///
/// The variant form lists an enum's variants; per variant it generates the
/// widening impls and narrowing accessors, plus the ownership bridge once
/// per invocation. The enum-to-enum form takes a nesting pair with the
/// shared variants and generates widening by variant remapping and
/// narrowing accessors by match.
///
/// # Syntax
///
/// ```ignore
/// morphism!(Subject { Iri, BlankNode });
/// morphism!(Subject => Term { Iri, BlankNode });
/// morphism!(IriNormalized as Iri => Subject);
/// ```
///
/// The source form widens a standalone type into an enum variant, converting
/// borrowed via `AsRef` and owned via `Into`.
#[macro_export]
macro_rules! morphism {
    ($Enum:ident { $($variant:ident),+ $(,)? }) => {
        $($crate::morphism!(@variant $Enum, $variant);)+
        $crate::morphism!(@bridge $Enum { $($variant),+ });
    };

    ($Narrow:ident => $Broad:ident { $($variant:ident),+ $(,)? }) => {
        $crate::paste::paste! {
            impl<'a> ::core::convert::From<$Narrow<'a>> for $Broad<'a> {
                fn from(value: $Narrow<'a>) -> Self {
                    match value {
                        $($Narrow::$variant(inner) => Self::$variant(inner),)+
                    }
                }
            }

            impl ::core::convert::From<[<$Narrow Buf>]> for [<$Broad Buf>] {
                fn from(value: [<$Narrow Buf>]) -> Self {
                    match value {
                        $([<$Narrow Buf>]::$variant(inner) => Self::$variant(inner),)+
                    }
                }
            }

            impl<'a> $Broad<'a> {
                #[allow(unreachable_patterns)]
                pub fn [<as_ $Narrow:snake>](self) -> ::core::option::Option<$Narrow<'a>> {
                    match self {
                        $($Broad::$variant(inner) => Some($Narrow::$variant(inner)),)+
                        _ => None,
                    }
                }
            }

            impl [<$Broad Buf>] {
                #[allow(unreachable_patterns)]
                pub fn [<into_ $Narrow:snake>](self) -> ::core::option::Option<[<$Narrow Buf>]> {
                    match self {
                        $([<$Broad Buf>]::$variant(inner) => Some([<$Narrow Buf>]::$variant(inner)),)+
                        _ => None,
                    }
                }
            }
        }
    };

    ($Source:ident as $variant:ident => $Enum:ident) => {
        $crate::paste::paste! {
            impl<'a> ::core::convert::From<&'a $Source> for $Enum<'a> {
                fn from(value: &'a $Source) -> Self {
                    Self::$variant(value.as_ref())
                }
            }

            impl ::core::convert::From<[<$Source Buf>]> for [<$Enum Buf>] {
                fn from(value: [<$Source Buf>]) -> Self {
                    Self::$variant(value.into())
                }
            }
        }
    };

    (@variant $Enum:ident, $variant:ident) => {
        $crate::paste::paste! {
            impl<'a> ::core::convert::From<&'a $variant> for $Enum<'a> {
                fn from(value: &'a $variant) -> Self {
                    Self::$variant(value)
                }
            }

            impl ::core::convert::From<[<$variant Buf>]> for [<$Enum Buf>] {
                fn from(value: [<$variant Buf>]) -> Self {
                    Self::$variant(value)
                }
            }

            impl<'a> $Enum<'a> {
                pub fn [<as_ $variant:snake>](self) -> ::core::option::Option<&'a $variant> {
                    match self {
                        Self::$variant(inner) => Some(inner),
                        _ => None,
                    }
                }
            }

            impl [<$Enum Buf>] {
                pub fn [<into_ $variant:snake>](self) -> ::core::option::Option<[<$variant Buf>]> {
                    match self {
                        Self::$variant(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        }
    };

    (@bridge $Enum:ident { $($variant:ident),+ }) => {
        $crate::paste::paste! {
            impl<'a> ::core::convert::From<&'a [<$Enum Buf>]> for $Enum<'a> {
                fn from(value: &'a [<$Enum Buf>]) -> Self {
                    match value {
                        $([<$Enum Buf>]::$variant(inner) => Self::$variant(inner),)+
                    }
                }
            }

            impl<'a> ::core::convert::From<$Enum<'a>> for [<$Enum Buf>] {
                fn from(value: $Enum<'a>) -> Self {
                    match value {
                        $($Enum::$variant(inner) => Self::$variant(inner.to_owned()),)+
                    }
                }
            }
        }
    };
}
