mod autolink;
mod link;
mod markdown;
mod wiki;

pub use autolink::Autolink;
pub use link::{ConversionError, Key, KeyBuf, Link, Target, ValidationError};
pub use markdown::{
    MarkdownDestination, MarkdownLink, MarkdownText, MarkdownTextBuf, MarkdownTitle,
    MarkdownTitleBuf,
};
pub use wiki::{
    WikiLink, WikiSection, WikiSectionBuf, WikiTarget, WikiTargetBuf, WikiText, WikiTextBuf,
};
