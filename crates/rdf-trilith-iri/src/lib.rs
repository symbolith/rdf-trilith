mod compat;
mod iri;
mod parse;
mod resolve;
mod validate;

pub use iri::{
    Authority, Base, BaseBuf, Fragment, Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized,
    IriNormalizedBuf, IriReference, IriReferenceBuf, IriRelative, IriRelativeBuf, Path, Query,
    Scheme, Uri, UriAbsolute, UriAbsoluteBuf, UriBase, UriBaseBuf, UriBuf, UriNormalized,
    UriNormalizedBuf, UriReference, UriReferenceBuf, UriRelative, UriRelativeBuf,
};
pub use resolve::resolve;
pub use validate::ValidationError;
