use crate::parse::{Components, parse};
use crate::{
    Authority, Base, IriAbsolute, IriBuf, IriReference, IriRelative, Query, UriAbsolute, UriBuf,
    UriReference, UriRelative,
};

impl IriReference {
    pub fn resolve(&self, base: impl AsRef<IriAbsolute>) -> IriBuf {
        resolve(base.as_ref(), self)
    }
}

impl IriAbsolute {
    pub fn resolve(&self, reference: impl AsRef<IriReference>) -> IriBuf {
        reference.as_ref().resolve(self)
    }
}

impl IriRelative {
    pub fn resolve(&self, base: impl AsRef<IriAbsolute>) -> IriBuf {
        let reference: &IriReference = self.as_ref();
        reference.resolve(base)
    }
}

impl UriReference {
    pub fn resolve(&self, base: impl AsRef<UriAbsolute>) -> UriBuf {
        #[allow(unsafe_code)]
        unsafe {
            let base = IriAbsolute::new_unchecked(base.as_ref());
            let reference = IriReference::new_unchecked(self);
            UriBuf::new_unchecked(resolve(base, reference).into())
        }
    }
}

impl UriAbsolute {
    pub fn resolve(&self, reference: impl AsRef<UriReference>) -> UriBuf {
        reference.as_ref().resolve(self)
    }
}

impl UriRelative {
    pub fn resolve(&self, base: impl AsRef<UriAbsolute>) -> UriBuf {
        let reference: &UriReference = self.as_ref();
        reference.resolve(base)
    }
}

fn components(validated: &str) -> Components<'_> {
    parse(validated).expect("validated on construction")
}

pub fn resolve(base: impl AsRef<Base>, reference: impl AsRef<IriReference>) -> IriBuf {
    let base = components(base.as_ref());
    let reference = components(reference.as_ref());
    let scheme = reference.scheme.or(base.scheme);
    let authority = target_authority(base, reference);
    let path = target_path(base, reference);
    let query = target_query(base, reference);
    let mut output = String::new();
    if let Some(scheme) = scheme {
        output.push_str(scheme);
        output.push(':');
    }
    if let Some(authority) = authority {
        output.push_str("//");
        output.push_str(authority);
    } else if path.starts_with("//") {
        output.push_str("/.");
    }
    output.push_str(&path);
    if let Some(query) = query {
        output.push('?');
        output.push_str(query);
    }
    if let Some(fragment) = reference.fragment {
        output.push('#');
        output.push_str(fragment);
    }
    #[allow(unsafe_code)]
    unsafe {
        IriBuf::new_unchecked(output)
    }
}

fn target_authority<'a>(base: Components<'a>, reference: Components<'a>) -> Option<&'a Authority> {
    if reference.scheme.is_some() {
        return reference.authority;
    }
    reference.authority.or(base.authority)
}

fn target_path(base: Components<'_>, reference: Components<'_>) -> String {
    let reference_path: &str = reference.path;
    if reference.scheme.is_some() || reference.authority.is_some() {
        return remove_dot_segments(reference_path);
    }
    if reference_path.is_empty() {
        let base_path: &str = base.path;
        return base_path.to_owned();
    }
    if reference_path.starts_with('/') {
        return remove_dot_segments(reference_path);
    }
    remove_dot_segments(&merge_paths(base, reference))
}

fn target_query<'a>(base: Components<'a>, reference: Components<'a>) -> Option<&'a Query> {
    let reference_path: &str = reference.path;
    let reference_is_empty =
        reference.scheme.is_none() && reference.authority.is_none() && reference_path.is_empty();
    if reference_is_empty {
        return reference.query.or(base.query);
    }
    reference.query
}

fn merge_paths(base: Components<'_>, reference: Components<'_>) -> String {
    let base_path: &str = base.path;
    let reference_path: &str = reference.path;
    if base.authority.is_some() && base_path.is_empty() {
        return format!("/{reference_path}");
    }
    match base_path.rfind('/') {
        Some(position) => format!("{}{}", &base_path[..=position], reference_path),
        None => reference_path.to_owned(),
    }
}

fn remove_dot_segments(path: &str) -> String {
    let mut input = path;
    let mut output = String::new();
    while !input.is_empty() {
        if let Some(rest) = input.strip_prefix("../") {
            input = rest;
        } else if let Some(rest) = input.strip_prefix("./") {
            input = rest;
        } else if input.starts_with("/./") {
            input = &input[2..];
        } else if input == "/." {
            input = "/";
        } else if input.starts_with("/../") {
            input = &input[3..];
            output.truncate(output.rfind('/').unwrap_or(0));
        } else if input == "/.." {
            input = "/";
            output.truncate(output.rfind('/').unwrap_or(0));
        } else if input == "." || input == ".." {
            input = "";
        } else {
            let start = usize::from(input.starts_with('/'));
            let segment_end = input[start..]
                .find('/')
                .map_or(input.len(), |position| position + start);
            output.push_str(&input[..segment_end]);
            input = &input[segment_end..];
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "http://a/b/c/d;p?q";

    fn resolved(base: &str, reference: &str) -> String {
        let base = IriAbsolute::new(base).unwrap();
        let reference = IriReference::new(reference).unwrap();
        resolve(base, reference).into()
    }

    #[test]
    fn rfc_normal_examples() {
        let cases = [
            ("g:h", "g:h"),
            ("g", "http://a/b/c/g"),
            ("./g", "http://a/b/c/g"),
            ("g/", "http://a/b/c/g/"),
            ("/g", "http://a/g"),
            ("//g", "http://g"),
            ("?y", "http://a/b/c/d;p?y"),
            ("g?y", "http://a/b/c/g?y"),
            ("#s", "http://a/b/c/d;p?q#s"),
            ("g#s", "http://a/b/c/g#s"),
            ("g?y#s", "http://a/b/c/g?y#s"),
            (";x", "http://a/b/c/;x"),
            ("g;x", "http://a/b/c/g;x"),
            ("g;x?y#s", "http://a/b/c/g;x?y#s"),
            ("", "http://a/b/c/d;p?q"),
            (".", "http://a/b/c/"),
            ("./", "http://a/b/c/"),
            ("..", "http://a/b/"),
            ("../", "http://a/b/"),
            ("../g", "http://a/b/g"),
            ("../..", "http://a/"),
            ("../../", "http://a/"),
            ("../../g", "http://a/g"),
        ];
        for (reference, expected) in cases {
            assert_eq!(resolved(BASE, reference), expected, "{reference}");
        }
    }

    #[test]
    fn rfc_abnormal_examples() {
        let cases = [
            ("../../../g", "http://a/g"),
            ("../../../../g", "http://a/g"),
            ("/./g", "http://a/g"),
            ("/../g", "http://a/g"),
            ("g.", "http://a/b/c/g."),
            (".g", "http://a/b/c/.g"),
            ("g..", "http://a/b/c/g.."),
            ("..g", "http://a/b/c/..g"),
            ("./../g", "http://a/b/g"),
            ("./g/.", "http://a/b/c/g/"),
            ("g/./h", "http://a/b/c/g/h"),
            ("g/../h", "http://a/b/c/h"),
            ("g;x=1/./y", "http://a/b/c/g;x=1/y"),
            ("g;x=1/../y", "http://a/b/c/y"),
            ("g?y/./x", "http://a/b/c/g?y/./x"),
            ("g?y/../x", "http://a/b/c/g?y/../x"),
            ("g#s/./x", "http://a/b/c/g#s/./x"),
            ("g#s/../x", "http://a/b/c/g#s/../x"),
            ("http:g", "http:g"),
        ];
        for (reference, expected) in cases {
            assert_eq!(resolved(BASE, reference), expected, "{reference}");
        }
    }

    #[test]
    fn rootless_base_merge() {
        assert_eq!(resolved("urn:example:animal", "test"), "urn:test");
    }

    #[test]
    fn authority_less_double_slash_guard() {
        assert_eq!(resolved("urn:/a/b", "..//x"), "urn:/.//x");
    }

    #[test]
    fn empty_reference_drops_base_fragment_slot() {
        assert_eq!(resolved("http://a/b?q", ""), "http://a/b?q");
        assert_eq!(resolved("http://a/b", "#f"), "http://a/b#f");
    }
}
