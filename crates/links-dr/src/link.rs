use iri_dr::{IriAbsolute, IriRelative};
use macro_validated_dr::{validated_str, validated_string};
use snafu::Snafu;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Link<'a> {
    pub target: Target<'a>,
    pub text: Option<&'a str>,
    pub title: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Target<'a> {
    Key(&'a Key),
    Relative(&'a IriRelative),
    Absolute(&'a IriAbsolute),
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum ValidationError {
    #[snafu(display("newline is not allowed"))]
    ContainsNewline,
    #[snafu(display("must not be empty"))]
    Empty,
    #[snafu(display("segment must not be empty"))]
    EmptySegment,
    #[snafu(display("character {character:?} is not allowed"))]
    ForbiddenCharacter { character: char },
    #[snafu(display("unescaped square brackets must be balanced"))]
    UnbalancedBrackets,
    #[snafu(display("backslash must escape an ASCII punctuation character"))]
    InvalidEscape,
    #[snafu(display("unescaped double quote is not allowed"))]
    UnescapedQuote,
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum ConversionError {
    #[snafu(display("target cannot be expressed in this link form"))]
    UnsupportedTarget,
    #[snafu(display("display text cannot be expressed in this link form"))]
    UnsupportedText,
    #[snafu(display("title cannot be expressed in this link form"))]
    UnsupportedTitle,
    #[snafu(transparent)]
    Component { source: ValidationError },
    #[snafu(transparent)]
    Iri { source: iri_dr::ValidationError },
}

validated_str!(pub Key, validate_key => ValidationError);
validated_string!(pub KeyBuf for Key, validate_key => ValidationError);

fn validate_key(input: &str) -> Result<(), ValidationError> {
    validate_newline_free(input)?;
    if input.is_empty() {
        return Err(ValidationError::Empty);
    }
    Ok(())
}

pub fn validate_newline_free(input: &str) -> Result<(), ValidationError> {
    if input.contains(['\n', '\r']) {
        return Err(ValidationError::ContainsNewline);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autolink::Autolink;
    use crate::markdown::{MarkdownDestination, MarkdownLink, MarkdownText, MarkdownTitle};
    use crate::wiki::{WikiLink, WikiSection, WikiTarget, WikiText};

    #[test]
    fn markdown_key_to_wiki() {
        let markdown = MarkdownLink {
            text: MarkdownText::new("a").unwrap(),
            destination: MarkdownDestination::Key(IriRelative::new("x#s").unwrap()),
            title: None,
        };
        let wiki = WikiLink::try_from(Link::from(markdown)).unwrap();
        assert_eq!(wiki.target.path(), "x");
        assert_eq!(wiki.target.section(), Some(WikiSection::new("s").unwrap()));
        assert_eq!(wiki.text, Some(WikiText::new("a").unwrap()));
    }

    #[test]
    fn wiki_to_markdown() {
        let wiki = WikiLink {
            target: WikiTarget::new("a/x#s").unwrap(),
            text: None,
        };
        let markdown = MarkdownLink::try_from(Link::from(wiki)).unwrap();
        assert_eq!(
            markdown.destination,
            MarkdownDestination::Key(IriRelative::new("a/x#s").unwrap())
        );
        assert_eq!(markdown.text, MarkdownText::new("").unwrap());
    }

    #[test]
    fn wiki_space_target_rejected_by_markdown() {
        let wiki = WikiLink {
            target: WikiTarget::new("a b").unwrap(),
            text: None,
        };
        assert!(matches!(
            MarkdownLink::try_from(Link::from(wiki)),
            Err(ConversionError::Iri { .. })
        ));
    }

    #[test]
    fn empty_key_destination_becomes_empty_target() {
        let markdown = MarkdownLink {
            text: MarkdownText::new("a").unwrap(),
            destination: MarkdownDestination::Key(IriRelative::new("").unwrap()),
            title: None,
        };
        assert_eq!(Link::from(markdown).target, Target::Empty);
    }

    #[test]
    fn autolink_round_trip() {
        let iri = IriAbsolute::new("https://x.y/b").unwrap();
        let link = Link::from(Autolink(iri));
        assert_eq!(Autolink::try_from(link).unwrap(), Autolink(iri));
        let markdown = MarkdownLink::try_from(link).unwrap();
        assert_eq!(markdown.destination, MarkdownDestination::Absolute(iri));
    }

    #[test]
    fn markdown_text_slice_crosses_forms_verbatim() {
        let markdown = MarkdownLink {
            text: MarkdownText::new(r"a\]b").unwrap(),
            destination: MarkdownDestination::Key(IriRelative::new("x").unwrap()),
            title: None,
        };
        let link = Link::from(markdown);
        assert_eq!(link.text, Some(r"a\]b"));
        assert_eq!(
            WikiLink::try_from(link),
            Err(ConversionError::Component {
                source: ValidationError::ForbiddenCharacter { character: ']' }
            })
        );
    }

    #[test]
    fn title_rejected_by_wiki_and_autolink() {
        let markdown = MarkdownLink {
            text: MarkdownText::new("a").unwrap(),
            destination: MarkdownDestination::Key(IriRelative::new("x").unwrap()),
            title: Some(MarkdownTitle::new("t").unwrap()),
        };
        let link = Link::from(markdown);
        assert_eq!(link.title, Some("t"));
        assert_eq!(
            WikiLink::try_from(link),
            Err(ConversionError::UnsupportedTitle)
        );
        assert_eq!(
            Autolink::try_from(link),
            Err(ConversionError::UnsupportedText)
        );
    }

    #[test]
    fn key_valid() {
        for input in ["x", "a/x", "a b#s", "a//b", "x#"] {
            assert_eq!(validate_key(input), Ok(()));
        }
    }

    #[test]
    fn key_invalid() {
        assert_eq!(validate_key(""), Err(ValidationError::Empty));
        assert_eq!(validate_key("a\nb"), Err(ValidationError::ContainsNewline));
    }
}
