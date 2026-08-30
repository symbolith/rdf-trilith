# links-dr overview

## Scope

- syntactic forms: markdown inline links, wiki links, autolinks (grammars in [link-forms.md](link-forms.md))
- parsing one link string into its typed structure
- a common link type every form converts into and can be created from
- out of scope: reference-style links `[a][ref]`, embeds `![[x]]` and `![a](x.png)`, GFM bare autolinks (`www.x.y`, `user@mail.org`), email autolinks (`<user@mail.org>`)

## Target cases

Every link target falls into exactly one of three cases, checked top to
bottom:

| Case     | Shape                                 | Examples                         | Resolution                                    |
|----------|---------------------------------------|----------------------------------|-----------------------------------------------|
| absolute | has a scheme                | `https://x.y/b`, `mailto:a@b.c`     | none; the target is already an IRI            |
| relative | everything else             | ``, `./x.md`, `x.md`, `a/x`, `x#s`  | RFC 3986 against a base, the current document |
| key      | bare name: no `/` `.` `#`   | `x`, `[[x]]`                        | resolver: look up the key |

## Semantics

Rows keyed by target case:

| Case     | Markdown                   | Wiki                | Autolink       | Navigation        | RDF export     |
|----------|----------------------------|---------------------|----------------|-------------------|----------------|
| absolute | `[a](https://x.y/b)`       | —                   | `<scheme:iri>` | browser / mail    | IRI verbatim   |
| relative | `[a](./x.md)`, `[a](x.md)` | `[[a/x]]`           | —              | file from doc     | base-resolved  |
| key      | `[a](x)`                   | `[[x]]`, `[[x\|a]]` | —              | via resolver      | via resolver   |

Non-target parts, independent of case:

| Part         | Markdown         | Wiki             | Autolink | Navigation     | RDF export   |
|--------------|------------------|------------------|----------|----------------|--------------|
| Display text | `[a]` (required) | `\|a` (optional) | —        | rendered label | not exported |
| Title        | `"title"`        | —                | —        | tooltip        | not exported |

## Compatibility

### Markdown links

| Case     | Shape                        | links-dr           | Browser/GitHub    | Obsidian shortest     | Obs. rel.             | Obs. abs.             |
|----------|------------------------------|--------------------|-------------------|-----------------------|-----------------------|-----------------------|
| absolute | `[a](scheme:iri)`            | verbatim           | verbatim          | verbatim              | verbatim              | verbatim              |
| relative | `[a](./x.md)`                | RFC 3986           | RFC 3986          | RFC 3986              | RFC 3986              | RFC 3986              |
| relative | `[a]()`                      | RFC 3986           | RFC 3986          | RFC 3986              | RFC 3986              | RFC 3986              |
| relative | `[a](#s)`                    | RFC 3986           | RFC 3986          | heading lookup in doc | heading lookup in doc | heading lookup in doc |
| relative | `[a](//x.y/a)`               | RFC 3986           | RFC 3986          | lookup, broken        | lookup, broken        | lookup, broken        |
| relative | `[a](a/x.md)`                | RFC 3986           | RFC 3986          | lookup                | lookup                | path from root        |
| relative | `[a](a/x)`                   | RFC 3986           | RFC 3986          | lookup, `.md` implied | lookup, `.md` implied | lookup, `.md` implied |
| relative | `[a](x.md)`                  | RFC 3986           | RFC 3986          | lookup                | lookup                | path from root        |
| relative | `[a](x#s)`                   | RFC 3986           | RFC 3986          | lookup + heading      | lookup + heading      | lookup + heading      |
| relative | `[a](x.md#s)`                | RFC 3986           | RFC 3986          | lookup + heading      | lookup + heading      | root + heading        |
| key      | `[a](x)`                     | resolve x          | RFC 3986          | lookup, `.md` implied | lookup, `.md` implied | lookup, `.md` implied |

### Wiki links

| Case | Shape             | links-dr           | Obsidian shortest | Obs. rel.        | Obs. abs.        | MediaWiki      |
|------|-------------------|--------------------|-------------------|------------------|------------------|----------------|
| relative | `[[a/x]]` | RFC 3986           | lookup, `.md` implied | lookup, `.md` implied | lookup, `.md` implied | page from root |
| relative | `[[x#s]]` | RFC 3986           | lookup + heading      | lookup + heading      | lookup + heading      | page from root + section |
| key      | `[[x]]`   | resolve x          | lookup, `.md` implied | lookup, `.md` implied | lookup, `.md` implied | page from root |

### Autolinks

| Case     | Shape          | links-dr | Browser/GitHub | Obsidian |
|----------|----------------|----------|----------------|----------|
| absolute | `<scheme:iri>` | verbatim | verbatim       | verbatim |

## Responsibilities

1. Parse: a link string into its typed structure (`MarkdownLink`, `WikiLink`, `Autolink`).
2. Unify: convert any parsed link into the common link type, and create any form from it.
3. Resolve, per case:
   - absolute: pass through
   - relative: RFC 3986 against the base
   - key: resolve IRI

<!-- vim: set conceallevel=1: -->
