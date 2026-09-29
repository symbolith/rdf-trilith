use crate::parse::{Components, parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, snafu::Snafu)]
pub enum ValidationError {
    #[snafu(display("non-ASCII character in URI"))]
    NotAscii,
    #[snafu(display("scheme is required but missing"))]
    MissingScheme,
    #[snafu(display("scheme is not allowed in a relative reference"))]
    ForbiddenScheme,
    #[snafu(display("fragment is not allowed in an absolute IRI"))]
    ForbiddenFragment,
    #[snafu(display("invalid scheme"))]
    InvalidScheme,
    #[snafu(display("invalid authority"))]
    InvalidAuthority,
    #[snafu(display("invalid path"))]
    InvalidPath,
    #[snafu(display("invalid query"))]
    InvalidQuery,
    #[snafu(display("invalid fragment"))]
    InvalidFragment,
    #[snafu(display("not in syntax-based normal form"))]
    NotNormalized,
}

#[derive(Clone, Copy, PartialEq)]
enum Family {
    Iri,
    Uri,
}

#[derive(Clone, Copy, PartialEq)]
enum SchemePolicy {
    Required,
    Optional,
    Forbidden,
}

#[derive(Clone, Copy, PartialEq)]
enum FragmentPolicy {
    Allowed,
    Forbidden,
}

fn validate(
    input: &str,
    family: Family,
    scheme_policy: SchemePolicy,
    fragment_policy: FragmentPolicy,
) -> Result<Components<'_>, ValidationError> {
    if family == Family::Uri && !input.is_ascii() {
        return Err(ValidationError::NotAscii);
    }
    let components = parse(input)?;
    match (components.scheme, scheme_policy) {
        (Some(_), SchemePolicy::Forbidden) => return Err(ValidationError::ForbiddenScheme),
        (None, SchemePolicy::Required) => return Err(ValidationError::MissingScheme),
        _ => {}
    }
    if components.fragment.is_some() && fragment_policy == FragmentPolicy::Forbidden {
        return Err(ValidationError::ForbiddenFragment);
    }
    Ok(components)
}

pub fn validate_iri(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Iri,
        SchemePolicy::Required,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_iri_reference(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Iri,
        SchemePolicy::Optional,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_iri_absolute(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Iri,
        SchemePolicy::Required,
        FragmentPolicy::Forbidden,
    )
}

pub fn validate_iri_relative(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Iri,
        SchemePolicy::Forbidden,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_uri(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Uri,
        SchemePolicy::Required,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_uri_reference(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Uri,
        SchemePolicy::Optional,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_uri_absolute(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Uri,
        SchemePolicy::Required,
        FragmentPolicy::Forbidden,
    )
}

pub fn validate_uri_relative(input: &str) -> Result<Components<'_>, ValidationError> {
    validate(
        input,
        Family::Uri,
        SchemePolicy::Forbidden,
        FragmentPolicy::Allowed,
    )
}

pub fn validate_iri_normalized(input: &str) -> Result<Components<'_>, ValidationError> {
    let components = validate_iri(input)?;
    validate_normal_form(input, &components)?;
    Ok(components)
}

pub fn validate_uri_normalized(input: &str) -> Result<Components<'_>, ValidationError> {
    let components = validate_uri(input)?;
    validate_normal_form(input, &components)?;
    Ok(components)
}

fn validate_normal_form(input: &str, components: &Components<'_>) -> Result<(), ValidationError> {
    if let Some(scheme) = components.scheme
        && scheme.bytes().any(|byte| byte.is_ascii_uppercase())
    {
        return Err(ValidationError::NotNormalized);
    }
    if let Some(authority) = components.authority
        && authority_host(authority)
            .bytes()
            .any(|byte| byte.is_ascii_uppercase())
    {
        return Err(ValidationError::NotNormalized);
    }
    if components
        .path
        .split('/')
        .any(|segment| matches!(segment, "." | ".."))
    {
        return Err(ValidationError::NotNormalized);
    }
    validate_percent_normal_form(input)
}

fn authority_host(authority: &str) -> &str {
    let host_and_port = match authority.find('@') {
        Some(position) => &authority[position + 1..],
        None => authority,
    };
    match host_and_port.strip_prefix('[') {
        Some(after_bracket) => after_bracket
            .find(']')
            .map_or(host_and_port, |end| &after_bracket[..end]),
        None => match host_and_port.find(':') {
            Some(position) => &host_and_port[..position],
            None => host_and_port,
        },
    }
}

fn validate_percent_normal_form(input: &str) -> Result<(), ValidationError> {
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let (Some(&high), Some(&low)) = (bytes.get(index + 1), bytes.get(index + 2)) else {
                return Err(ValidationError::NotNormalized);
            };
            if high.is_ascii_lowercase() || low.is_ascii_lowercase() {
                return Err(ValidationError::NotNormalized);
            }
            let decoded = hexadecimal_value(high) * 16 + hexadecimal_value(low);
            if decoded.is_ascii_alphanumeric() || matches!(decoded, b'-' | b'.' | b'_' | b'~') {
                return Err(ValidationError::NotNormalized);
            }
            index += 3;
        } else {
            index += 1;
        }
    }
    Ok(())
}

fn hexadecimal_value(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'A'..=b'F' => digit - b'A' + 10,
        b'a'..=b'f' => digit - b'a' + 10,
        _ => 0,
    }
}

pub fn validate_scheme(input: &str) -> Result<(), ValidationError> {
    let mut characters = input.chars();
    let valid = characters.next().is_some_and(|c| c.is_ascii_alphabetic())
        && characters.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    valid.then_some(()).ok_or(ValidationError::InvalidScheme)
}

pub fn validate_authority(input: &str) -> Result<(), ValidationError> {
    let (userinfo, host_and_port) = match input.find('@') {
        Some(position) => (Some(&input[..position]), &input[position + 1..]),
        None => (None, input),
    };
    if let Some(userinfo) = userinfo {
        validate_characters(
            userinfo,
            |c| is_unreserved(c) || is_sub_delimiter(c) || c == ':' || is_ucschar(c),
            ValidationError::InvalidAuthority,
        )?;
    }
    let (host, port) = if let Some(after_bracket) = host_and_port.strip_prefix('[') {
        let Some(end) = after_bracket.find(']') else {
            return Err(ValidationError::InvalidAuthority);
        };
        let literal = &after_bracket[..end];
        let literal_valid = if literal.starts_with(['v', 'V']) {
            literal[1..]
                .chars()
                .all(|c| is_unreserved(c) || is_sub_delimiter(c) || matches!(c, ':' | '.'))
        } else {
            !literal.is_empty()
                && literal
                    .chars()
                    .all(|c| c.is_ascii_hexdigit() || matches!(c, ':' | '.'))
        };
        if !literal_valid {
            return Err(ValidationError::InvalidAuthority);
        }
        let remainder = &after_bracket[end + 1..];
        match remainder.strip_prefix(':') {
            Some(port) => (None, Some(port)),
            None if remainder.is_empty() => (None, None),
            None => return Err(ValidationError::InvalidAuthority),
        }
    } else {
        match host_and_port.find(':') {
            Some(position) => (
                Some(&host_and_port[..position]),
                Some(&host_and_port[position + 1..]),
            ),
            None => (Some(host_and_port), None),
        }
    };
    if let Some(host) = host {
        validate_characters(
            host,
            |c| is_unreserved(c) || is_sub_delimiter(c) || is_ucschar(c),
            ValidationError::InvalidAuthority,
        )?;
    }
    if let Some(port) = port
        && !port.chars().all(|c| c.is_ascii_digit())
    {
        return Err(ValidationError::InvalidAuthority);
    }
    Ok(())
}

pub fn validate_path(input: &str) -> Result<(), ValidationError> {
    validate_characters(
        input,
        |c| is_pchar(c) || c == '/',
        ValidationError::InvalidPath,
    )
}

pub fn validate_query(input: &str) -> Result<(), ValidationError> {
    validate_characters(
        input,
        |c| is_pchar(c) || matches!(c, '/' | '?') || is_iprivate(c),
        ValidationError::InvalidQuery,
    )
}

pub fn validate_fragment(input: &str) -> Result<(), ValidationError> {
    validate_characters(
        input,
        |c| is_pchar(c) || matches!(c, '/' | '?'),
        ValidationError::InvalidFragment,
    )
}

fn is_unreserved(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '-' | '.' | '_' | '~')
}

fn is_sub_delimiter(character: char) -> bool {
    matches!(
        character,
        '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
    )
}

fn is_ucschar(character: char) -> bool {
    let code_point = u32::from(character);
    matches!(
        code_point,
        0xA0..=0xD7FF | 0xF900..=0xFDCF | 0xFDF0..=0xFFEF | 0xE1000..=0xEFFFD
    ) || ((0x1_0000..=0xD_FFFD).contains(&code_point) && code_point & 0xFFFF <= 0xFFFD)
}

fn is_iprivate(character: char) -> bool {
    let code_point = u32::from(character);
    matches!(
        code_point,
        0xE000..=0xF8FF | 0xF_0000..=0xF_FFFD | 0x10_0000..=0x10_FFFD
    )
}

fn is_pchar(character: char) -> bool {
    is_unreserved(character)
        || is_sub_delimiter(character)
        || matches!(character, ':' | '@')
        || is_ucschar(character)
}

fn validate_characters(
    segment: &str,
    is_allowed: impl Fn(char) -> bool,
    error: ValidationError,
) -> Result<(), ValidationError> {
    let mut characters = segment.chars();
    while let Some(character) = characters.next() {
        if character == '%' {
            let valid = characters.next().is_some_and(|c| c.is_ascii_hexdigit())
                && characters.next().is_some_and(|c| c.is_ascii_hexdigit());
            if !valid {
                return Err(error);
            }
        } else if !is_allowed(character) {
            return Err(error);
        }
    }
    Ok(())
}
