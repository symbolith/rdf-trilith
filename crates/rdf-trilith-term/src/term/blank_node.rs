use rdf_trilith_macro_validated::{validated_str, validated_string};

use crate::ValidationError;

validated_str!(pub BlankNode, validate_blank_node_label => ValidationError);
validated_string!(pub BlankNodeBuf for BlankNode, validate_blank_node_label => ValidationError);

fn validate_blank_node_label(input: &str) -> Result<(), ValidationError> {
    let mut characters = input.chars();
    let (Some(first), Some(last)) = (characters.next(), input.chars().next_back()) else {
        return Err(ValidationError::InvalidBlankNodeLabel);
    };
    let valid = is_label_start(first)
        && characters.all(|character| is_pn_chars(character) || character == '.')
        && is_pn_chars(last);
    if valid {
        Ok(())
    } else {
        Err(ValidationError::InvalidBlankNodeLabel)
    }
}

fn is_label_start(character: char) -> bool {
    character.is_ascii_digit() || is_pn_chars_u(character)
}

fn is_pn_chars_u(character: char) -> bool {
    character == '_' || is_pn_chars_base(character)
}

fn is_pn_chars(character: char) -> bool {
    is_pn_chars_u(character)
        || matches!(character,
            '-' | '0'..='9' | '\u{B7}' | '\u{300}'..='\u{36F}' | '\u{203F}'..='\u{2040}')
}

fn is_pn_chars_base(character: char) -> bool {
    matches!(character,
        'A'..='Z' | 'a'..='z'
        | '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}'
        | '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_node_labels_valid() {
        for label in ["a", "0", "_", "b0", "0b", "_x", "a.b", "a-b", "a\u{B7}b"] {
            assert!(validate_blank_node_label(label).is_ok(), "{label}");
        }
    }

    #[test]
    fn blank_node_labels_invalid() {
        for label in ["", ".", "a.", ".a", "-a", "a b", "a!"] {
            assert!(validate_blank_node_label(label).is_err(), "{label}");
        }
    }
}
