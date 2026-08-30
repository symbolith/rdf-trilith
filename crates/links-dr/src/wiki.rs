use macro_validated_dr::{validated_str, validated_string};

use crate::link::{ConversionError, Key, Link, Target, ValidationError, validate_newline_free};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WikiLink<'a> {
    pub target: &'a WikiTarget,
    pub text: Option<&'a WikiText>,
}

validated_str!(pub WikiTarget, validate_wiki_target => ValidationError);
validated_string!(pub WikiTargetBuf for WikiTarget, validate_wiki_target => ValidationError);

impl WikiTarget {
    pub fn path(&self) -> &str {
        let target: &str = self;
        target.split_once('#').map_or(target, |(path, _)| path)
    }

    pub fn section(&self) -> Option<&WikiSection> {
        let target: &str = self;
        target
            .split_once('#')
            .map(|(_, section)| WikiSection::new(section).expect("validated with the target"))
    }
}

fn validate_wiki_target(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    let (path, section) = match input.split_once('#') {
        Some((path, section)) => (path, Some(section)),
        None => (input, None),
    };
    if path.is_empty() {
        return Err(ValidationError::Empty);
    }
    for segment in path.split('/') {
        if segment.is_empty() {
            return Err(ValidationError::EmptySegment);
        }
        validate_no_forbidden_characters(segment, &['|', '[', ']'])?;
    }
    match section {
        Some(section) => validate_wiki_section(section),
        None => Ok(()),
    }
}

validated_str!(pub WikiSection, validate_wiki_section => ValidationError);
validated_string!(pub WikiSectionBuf for WikiSection, validate_wiki_section => ValidationError);

fn validate_wiki_section(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    if input.is_empty() {
        return Err(ValidationError::Empty);
    }
    validate_no_forbidden_characters(input, &['|', '[', ']'])
}

validated_str!(pub WikiText, validate_wiki_text => ValidationError);
validated_string!(pub WikiTextBuf for WikiText, validate_wiki_text => ValidationError);

fn validate_wiki_text(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    if input.is_empty() {
        return Err(ValidationError::Empty);
    }
    validate_no_forbidden_characters(input, &['[', ']'])
}

fn validate_no_forbidden_characters(
    input: &str,
    forbidden: &[char],
) -> Result<(), ValidationError> {
    match input
        .chars()
        .find(|character| forbidden.contains(character))
    {
        Some(character) => Err(ValidationError::ForbiddenCharacter { character }),
        None => Ok(()),
    }
}

impl<'a> From<WikiLink<'a>> for Link<'a> {
    fn from(link: WikiLink<'a>) -> Self {
        Link {
            target: Target::Key(
                Key::new(link.target).expect("a wiki target is non-empty and newline-free"),
            ),
            text: link.text.map(|text| -> &str { text }),
            title: None,
        }
    }
}

impl<'a> TryFrom<Link<'a>> for WikiLink<'a> {
    type Error = ConversionError;

    fn try_from(link: Link<'a>) -> Result<Self, ConversionError> {
        if link.title.is_some() {
            return Err(ConversionError::UnsupportedTitle);
        }
        let Target::Key(key) = link.target else {
            return Err(ConversionError::UnsupportedTarget);
        };
        Ok(WikiLink {
            target: WikiTarget::new(key)?,
            text: link.text.map(WikiText::new).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_target_valid() {
        for input in ["x", "a/x", "a b/x y", "x.y", "x#s", "a/x#s b", "x#s#t"] {
            assert_eq!(validate_wiki_target(input), Ok(()));
        }
    }

    #[test]
    fn wiki_target_invalid() {
        assert_eq!(validate_wiki_target(""), Err(ValidationError::Empty));
        assert_eq!(validate_wiki_target("#s"), Err(ValidationError::Empty));
        assert_eq!(validate_wiki_target("x#"), Err(ValidationError::Empty));
        assert_eq!(
            validate_wiki_target("/x"),
            Err(ValidationError::EmptySegment)
        );
        assert_eq!(
            validate_wiki_target("a/"),
            Err(ValidationError::EmptySegment)
        );
        assert_eq!(
            validate_wiki_target("a//x"),
            Err(ValidationError::EmptySegment)
        );
        for (input, character) in [("a|b", '|'), ("a[b", '['), ("a]b", ']'), ("x#s|t", '|')] {
            assert_eq!(
                validate_wiki_target(input),
                Err(ValidationError::ForbiddenCharacter { character })
            );
        }
    }

    #[test]
    fn wiki_section_valid() {
        for input in ["s", "a#b", "a/b"] {
            assert_eq!(validate_wiki_section(input), Ok(()));
        }
    }

    #[test]
    fn wiki_section_invalid() {
        assert_eq!(validate_wiki_section(""), Err(ValidationError::Empty));
        for (input, character) in [("a|b", '|'), ("a[b", '['), ("a]b", ']')] {
            assert_eq!(
                validate_wiki_section(input),
                Err(ValidationError::ForbiddenCharacter { character })
            );
        }
    }

    #[test]
    fn wiki_text_valid() {
        for input in ["a", "a|b", "a#b"] {
            assert_eq!(validate_wiki_text(input), Ok(()));
        }
    }

    #[test]
    fn wiki_text_invalid() {
        assert_eq!(validate_wiki_text(""), Err(ValidationError::Empty));
        for (input, character) in [("a[b", '['), ("a]b", ']')] {
            assert_eq!(
                validate_wiki_text(input),
                Err(ValidationError::ForbiddenCharacter { character })
            );
        }
    }
}
