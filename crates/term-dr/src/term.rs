mod blank_node;
mod literal;

pub use blank_node::{BlankNode, BlankNodeBuf};
pub use literal::{LanguageTag, LanguageTagBuf, LexicalForm, LexicalFormBuf, Literal, LiteralBuf};

use macro_enum_dr::morphism;

use crate::{Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized, IriNormalizedBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Term<'a> {
    Iri(&'a Iri),
    BlankNode(&'a BlankNode),
    Literal(&'a Literal),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TermBuf {
    Iri(IriBuf),
    BlankNode(BlankNodeBuf),
    Literal(LiteralBuf),
}

morphism!(Term {
    Iri,
    BlankNode,
    Literal
});
morphism!(IriNormalized as Iri => Term);
morphism!(IriAbsolute as Iri => Term);
