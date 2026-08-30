use macro_validated_dr::{validated_str, validated_string};

use crate::ValidationError;

validated_str!(pub LanguageTag, validate_language_tag => ValidationError);
validated_string!(pub LanguageTagBuf for LanguageTag, validate_language_tag => ValidationError);

fn validate_language_tag(input: &str) -> Result<(), ValidationError> {
    if is_well_formed_language_tag(input) {
        Ok(())
    } else {
        Err(ValidationError::InvalidLanguageTag)
    }
}

type Subtags<'a> = std::iter::Peekable<std::str::Split<'a, char>>;

const GRANDFATHERED: [&str; 26] = [
    "en-GB-oed",
    "i-ami",
    "i-bnn",
    "i-default",
    "i-enochian",
    "i-hak",
    "i-klingon",
    "i-lux",
    "i-mingo",
    "i-navajo",
    "i-pwn",
    "i-tao",
    "i-tay",
    "i-tsu",
    "sgn-BE-FR",
    "sgn-BE-NL",
    "sgn-CH-DE",
    "art-lojban",
    "cel-gaulish",
    "no-bok",
    "no-nyn",
    "zh-guoyu",
    "zh-hakka",
    "zh-min",
    "zh-min-nan",
    "zh-xiang",
];

fn is_well_formed_language_tag(input: &str) -> bool {
    if input.is_empty()
        || !input
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return false;
    }
    if GRANDFATHERED
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(input))
    {
        return true;
    }
    let mut subtags = input.split('-').peekable();
    let well_formed = if consume_if(&mut subtags, is_private_use_marker) {
        private_use(&mut subtags)
    } else {
        language_tag(&mut subtags)
    };
    well_formed && subtags.next().is_none()
}

fn language_tag(subtags: &mut Subtags) -> bool {
    if !primary_language(subtags) {
        return false;
    }
    consume_if(subtags, is_script);
    consume_if(subtags, is_region);
    while consume_if(subtags, is_variant) {}
    while next_is(subtags, is_singleton) {
        if !extension(subtags) {
            return false;
        }
    }
    if consume_if(subtags, is_private_use_marker) {
        return private_use(subtags);
    }
    true
}

fn primary_language(subtags: &mut Subtags) -> bool {
    let Some(language) = subtags.next() else {
        return false;
    };
    if !is_alpha(language) {
        return false;
    }
    match language.len() {
        2..=3 => {
            for _ in 0..3 {
                if !consume_if(subtags, is_extended_language) {
                    break;
                }
            }
            true
        }
        4..=8 => true,
        _ => false,
    }
}

fn extension(subtags: &mut Subtags) -> bool {
    subtags.next();
    let mut count = 0;
    while consume_if(subtags, is_extension_subtag) {
        count += 1;
    }
    count > 0
}

fn private_use(subtags: &mut Subtags) -> bool {
    let mut count = 0;
    while consume_if(subtags, is_private_use_subtag) {
        count += 1;
    }
    count > 0
}

fn next_is(subtags: &mut Subtags, predicate: fn(&str) -> bool) -> bool {
    subtags.peek().copied().is_some_and(predicate)
}

fn consume_if(subtags: &mut Subtags, predicate: fn(&str) -> bool) -> bool {
    let matches = next_is(subtags, predicate);
    if matches {
        subtags.next();
    }
    matches
}

fn is_alpha(subtag: &str) -> bool {
    !subtag.is_empty() && subtag.bytes().all(|byte| byte.is_ascii_alphabetic())
}

fn is_digits(subtag: &str) -> bool {
    !subtag.is_empty() && subtag.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_extended_language(subtag: &str) -> bool {
    subtag.len() == 3 && is_alpha(subtag)
}

fn is_script(subtag: &str) -> bool {
    subtag.len() == 4 && is_alpha(subtag)
}

fn is_region(subtag: &str) -> bool {
    (subtag.len() == 2 && is_alpha(subtag)) || (subtag.len() == 3 && is_digits(subtag))
}

fn is_variant(subtag: &str) -> bool {
    matches!(subtag.len(), 5..=8) || (subtag.len() == 4 && subtag.as_bytes()[0].is_ascii_digit())
}

fn is_singleton(subtag: &str) -> bool {
    subtag.len() == 1 && !subtag.eq_ignore_ascii_case("x")
}

fn is_private_use_marker(subtag: &str) -> bool {
    subtag.eq_ignore_ascii_case("x")
}

fn is_extension_subtag(subtag: &str) -> bool {
    matches!(subtag.len(), 2..=8)
}

fn is_private_use_subtag(subtag: &str) -> bool {
    matches!(subtag.len(), 1..=8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_tags_valid() {
        for tag in [
            "en",
            "EN",
            "en-US",
            "zh-Hant-TW",
            "de-CH-1901",
            "sl-rozaj-biske",
            "es-419",
            "x-private",
            "en-a-bbb-x-a-b",
            "i-klingon",
            "en-GB-oed",
            "mn-Cyrl-MN",
            "en-x-us",
            "az-Arab-x-AZE-derbend",
        ] {
            assert!(validate_language_tag(tag).is_ok(), "{tag}");
        }
    }

    #[test]
    fn language_tags_invalid() {
        for tag in [
            "",
            "-en",
            "en-",
            "en--us",
            "a",
            "x",
            "en-a",
            "en-x",
            "abcd-abc",
            "123",
            "en-999-a",
            "i-notregistered",
        ] {
            assert!(validate_language_tag(tag).is_err(), "{tag}");
        }
    }
}
