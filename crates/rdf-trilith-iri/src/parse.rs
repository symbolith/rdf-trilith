use crate::validate::ValidationError;
use crate::{Authority, Fragment, Path, Query, Scheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Components<'a> {
    pub scheme: Option<&'a Scheme>,
    pub authority: Option<&'a Authority>,
    pub path: &'a Path,
    pub query: Option<&'a Query>,
    pub fragment: Option<&'a Fragment>,
}

pub fn parse(input: &str) -> Result<Components<'_>, ValidationError> {
    let scheme = input
        .find([':', '/', '?', '#'])
        .filter(|&position| input[position..].starts_with(':'))
        .map(|position| &input[..position]);
    let after_scheme = match scheme {
        Some(scheme) => &input[scheme.len() + 1..],
        None => input,
    };
    let (before_fragment, fragment) = match after_scheme.find('#') {
        Some(position) => (
            &after_scheme[..position],
            Some(&after_scheme[position + 1..]),
        ),
        None => (after_scheme, None),
    };
    let (before_query, query) = match before_fragment.find('?') {
        Some(position) => (
            &before_fragment[..position],
            Some(&before_fragment[position + 1..]),
        ),
        None => (before_fragment, None),
    };
    let (authority, path) = match before_query.strip_prefix("//") {
        Some(after) => {
            let path_start = after.find('/').unwrap_or(after.len());
            (Some(&after[..path_start]), &after[path_start..])
        }
        None => (None, before_query),
    };
    Ok(Components {
        scheme: scheme.map(Scheme::new).transpose()?,
        authority: authority.map(Authority::new).transpose()?,
        path: Path::new(path)?,
        query: query.map(Query::new).transpose()?,
        fragment: fragment.map(Fragment::new).transpose()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text<T: AsRef<str> + ?Sized>(value: &T) -> &str {
        value.as_ref()
    }

    #[test]
    fn full() {
        let components = parse("foo://example.com:8042/over/there?name=ferret#nose").unwrap();
        assert_eq!(components.scheme.map(text), Some("foo"));
        assert_eq!(components.authority.map(text), Some("example.com:8042"));
        assert_eq!(text(components.path), "/over/there");
        assert_eq!(components.query.map(text), Some("name=ferret"));
        assert_eq!(components.fragment.map(text), Some("nose"));
    }

    #[test]
    fn no_scheme_before_delimiter() {
        let components = parse("//host/path?q#f").unwrap();
        assert_eq!(components.scheme, None);
        assert_eq!(components.authority.map(text), Some("host"));
        assert_eq!(text(components.path), "/path");
        assert_eq!(components.query.map(text), Some("q"));
        assert_eq!(components.fragment.map(text), Some("f"));
    }

    #[test]
    fn urn_style() {
        let components = parse("urn:example:animal:ferret:nose").unwrap();
        assert_eq!(components.scheme.map(text), Some("urn"));
        assert_eq!(components.authority, None);
        assert_eq!(text(components.path), "example:animal:ferret:nose");
        assert_eq!(components.query, None);
        assert_eq!(components.fragment, None);
    }

    #[test]
    fn empty() {
        let components = parse("").unwrap();
        assert_eq!(components.scheme, None);
        assert_eq!(components.authority, None);
        assert_eq!(text(components.path), "");
        assert_eq!(components.query, None);
        assert_eq!(components.fragment, None);
    }

    #[test]
    fn authority_without_path() {
        let components = parse("http://example.com").unwrap();
        assert_eq!(components.authority.map(text), Some("example.com"));
        assert_eq!(text(components.path), "");
        assert_eq!(components.fragment, None);
    }

    #[test]
    fn empty_components_are_present() {
        let components = parse("http://x/?#").unwrap();
        assert_eq!(components.authority.map(text), Some("x"));
        assert_eq!(text(components.path), "/");
        assert_eq!(components.query.map(text), Some(""));
        assert_eq!(components.fragment.map(text), Some(""));
    }

    #[test]
    fn fragment_before_query_delimiter() {
        let components = parse("a#b?c").unwrap();
        assert_eq!(text(components.path), "a");
        assert_eq!(components.query, None);
        assert_eq!(components.fragment.map(text), Some("b?c"));
    }

    #[test]
    fn invalid_components() {
        assert_eq!(
            parse("1http://x").unwrap_err(),
            ValidationError::InvalidScheme
        );
        assert_eq!(
            parse("http://ex ample/").unwrap_err(),
            ValidationError::InvalidAuthority,
        );
        assert_eq!(
            parse("http://x/a b").unwrap_err(),
            ValidationError::InvalidPath
        );
    }
}
