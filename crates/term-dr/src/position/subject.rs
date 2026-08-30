use macro_enum_dr::morphism;

use crate::{
    BlankNode, BlankNodeBuf, Iri, IriAbsolute, IriAbsoluteBuf, IriBuf, IriNormalized,
    IriNormalizedBuf, Term, TermBuf,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Subject<'a> {
    Iri(&'a Iri),
    BlankNode(&'a BlankNode),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubjectBuf {
    Iri(IriBuf),
    BlankNode(BlankNodeBuf),
}

morphism!(Subject { Iri, BlankNode });
morphism!(IriNormalized as Iri => Subject);
morphism!(IriAbsolute as Iri => Subject);
morphism!(Subject => Term { Iri, BlankNode });
