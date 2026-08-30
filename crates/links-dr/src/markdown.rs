use iri_dr::{IriAbsolute, IriRelative};
use macro_validated_dr::{validated_str, validated_string};

use crate::link::{ConversionError, Key, Link, Target, ValidationError, validate_newline_free};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct MarkdownLink<'a> {
    pub text: &'a MarkdownText,
    pub destination: MarkdownDestination<'a>,
    pub title: Option<&'a MarkdownTitle>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MarkdownDestination<'a> {
    Key(&'a IriRelative),
    Relative(&'a IriRelative),
    Absolute(&'a IriAbsolute),
    Empty,
}

validated_str!(pub MarkdownText, validate_markdown_text => ValidationError);
validated_string!(pub MarkdownTextBuf for MarkdownText, validate_markdown_text => ValidationError);

fn validate_markdown_text(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    let mut depth: usize = 0;
    let mut characters = input.chars();
    while let Some(character) = characters.next() {
        match character {
            '\\' => match characters.next() {
                Some(escaped) if escaped.is_ascii_punctuation() => {}
                _ => return Err(ValidationError::InvalidEscape),
            },
            '[' => depth += 1,
            ']' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or(ValidationError::UnbalancedBrackets)?;
            }
            _ => {}
        }
    }
    if depth == 0 {
        Ok(())
    } else {
        Err(ValidationError::UnbalancedBrackets)
    }
}

validated_str!(pub MarkdownTitle, validate_markdown_title => ValidationError);
validated_string!(pub MarkdownTitleBuf for MarkdownTitle, validate_markdown_title => ValidationError);

fn validate_markdown_title(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    let mut characters = input.chars();
    while let Some(character) = characters.next() {
        match character {
            '\\' => match characters.next() {
                Some(escaped) if escaped.is_ascii_punctuation() => {}
                _ => return Err(ValidationError::InvalidEscape),
            },
            '"' => return Err(ValidationError::UnescapedQuote),
            _ => {}
        }
    }
    Ok(())
}

impl<'a> From<MarkdownLink<'a>> for Link<'a> {
    fn from(link: MarkdownLink<'a>) -> Self {
        let target = match link.destination {
            MarkdownDestination::Key(iri) if iri.is_empty() => Target::Empty,
            MarkdownDestination::Key(iri) => {
                Target::Key(Key::new(iri).expect("a non-empty IRI reference is a valid key"))
            }
            MarkdownDestination::Relative(iri) => Target::Relative(iri),
            MarkdownDestination::Absolute(iri) => Target::Absolute(iri),
            MarkdownDestination::Empty => Target::Empty,
        };
        Link {
            target,
            text: Some(link.text),
            title: link.title.map(|title| -> &str { title }),
        }
    }
}

impl<'a> TryFrom<Link<'a>> for MarkdownLink<'a> {
    type Error = ConversionError;

    fn try_from(link: Link<'a>) -> Result<Self, ConversionError> {
        let destination = match link.target {
            Target::Key(key) => MarkdownDestination::Key(IriRelative::new(key)?),
            Target::Relative(iri) => MarkdownDestination::Relative(iri),
            Target::Absolute(iri) => MarkdownDestination::Absolute(iri),
            Target::Empty => MarkdownDestination::Empty,
        };
        let text = match link.text {
            Some(text) => MarkdownText::new(text)?,
            None => MarkdownText::new("").expect("empty markdown text is valid"),
        };
        Ok(MarkdownLink {
            text,
            destination,
            title: link.title.map(MarkdownTitle::new).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_text_valid() {
        for input in ["", "plain", "a[b]c", "[[x]]", r"a\]b", r"\[", r"\\"] {
            assert_eq!(validate_markdown_text(input), Ok(()));
        }
    }

    #[test]
    fn markdown_text_invalid() {
        assert_eq!(
            validate_markdown_text("a]b"),
            Err(ValidationError::UnbalancedBrackets)
        );
        assert_eq!(
            validate_markdown_text("[a"),
            Err(ValidationError::UnbalancedBrackets)
        );
        assert_eq!(
            validate_markdown_text(r"a\x"),
            Err(ValidationError::InvalidEscape)
        );
        assert_eq!(
            validate_markdown_text("a\\"),
            Err(ValidationError::InvalidEscape)
        );
        assert_eq!(
            validate_markdown_text("a\nb"),
            Err(ValidationError::ContainsNewline)
        );
    }

    #[test]
    fn markdown_title_valid() {
        for input in ["", "plain", r#"a\"b"#, "'single'", "(paren)"] {
            assert_eq!(validate_markdown_title(input), Ok(()));
        }
    }

    #[test]
    fn markdown_title_invalid() {
        assert_eq!(
            validate_markdown_title(r#"a"b"#),
            Err(ValidationError::UnescapedQuote)
        );
        assert_eq!(
            validate_markdown_title("a\\"),
            Err(ValidationError::InvalidEscape)
        );
        assert_eq!(
            validate_markdown_title(r"a\x"),
            Err(ValidationError::InvalidEscape)
        );
        assert_eq!(
            validate_markdown_title("a\nb"),
            Err(ValidationError::ContainsNewline)
        );
    }
}
