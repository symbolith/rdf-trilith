use rdf_trilith_macro_validated::{validated_str, validated_string};

use crate::ValidationError;

validated_str!(pub LexicalForm, validate_lexical_form => ValidationError);
validated_string!(pub LexicalFormBuf for LexicalForm, validate_lexical_form => ValidationError);

fn validate_lexical_form(_input: &str) -> Result<(), ValidationError> {
    Ok(())
}
