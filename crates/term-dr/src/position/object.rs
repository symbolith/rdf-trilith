use macro_enum_dr::morphism;

use crate::{
    BlankNode, BlankNodeBuf, Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized,
    IriNormalizedBuf, Literal, LiteralBuf, Subject, SubjectBuf, Term, TermBuf,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Object<'a> {
    Iri(&'a Iri),
    BlankNode(&'a BlankNode),
    Literal(&'a Literal),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectBuf {
    Iri(IriBuf),
    BlankNode(BlankNodeBuf),
    Literal(LiteralBuf),
}

morphism!(Object {
    Iri,
    BlankNode,
    Literal
});
morphism!(IriNormalized as Iri => Object);
morphism!(IriAbsolute as Iri => Object);
morphism!(Subject => Object { Iri, BlankNode });
morphism!(Object => Term { Iri, BlankNode, Literal });
