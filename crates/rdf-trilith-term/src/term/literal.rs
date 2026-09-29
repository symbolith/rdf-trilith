mod language_tag;
mod lexical_form;

pub use language_tag::{LanguageTag, LanguageTagBuf};
pub use lexical_form::{LexicalForm, LexicalFormBuf};

use crate::IriBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    Simple {
        lexical_form: LexicalFormBuf,
    },
    Typed {
        lexical_form: LexicalFormBuf,
        datatype_iri: IriBuf,
    },
    LanguageTagged {
        lexical_form: LexicalFormBuf,
        language_tag: LanguageTagBuf,
    },
}

pub type LiteralBuf = Literal;
