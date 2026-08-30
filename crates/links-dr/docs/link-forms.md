# Link forms

W3C EBNF notation. Terminals uppercase.

```
ANY_CHAR   ::= [#x0-#x10FFFF]
SP         ::= #x20
WS         ::= (#x20 | #x9 | #xA | #xD)+
NEWLINE    ::= #xA | #xD
CONTROL    ::= [#x0-#x1F] | #x7F
HEX        ::= [0-9A-Fa-f]
ASCII_PUNCT ::= [!-/] | [:-@] | [[-`] | [{-~]
```

## Markdown inline links

CommonMark 0.31.2 §6.3.

```
inline-link  ::= '[' link-text ']' '(' WS? destination? (WS title)? WS? ')'
link-text    ::= (LINK_TEXT_CHAR | '\' ASCII_PUNCT | '[' link-text ']')*
destination  ::= '<' DEST_BRACKET_CHAR* '>'
               | DEST_PLAIN_CHAR+

title        ::= '"' (ANY_CHAR - '"' | '\' ASCII_PUNCT)* '"'
               | "'" (ANY_CHAR - "'" | '\' ASCII_PUNCT)* "'"
               | '(' (ANY_CHAR - ('(' | ')') | '\' ASCII_PUNCT)* ')'

LINK_TEXT_CHAR    ::= ANY_CHAR - ('[' | ']' | '\')
DEST_BRACKET_CHAR ::= (ANY_CHAR - ('<' | '>' | NEWLINE | '\')) | '\' ASCII_PUNCT
DEST_PLAIN_CHAR   ::= (ANY_CHAR - (SP | CONTROL | '\')) | '\' ASCII_PUNCT
```

Reduced:

```
[text](destination "title")
```

## Autolinks

CommonMark 0.31.2 §6.5. Out of scope: GFM §6.9 bare autolinks (`https://bare.url`, `www.bare.url`, `user@mail.org`) and CommonMark email autolinks (`<user@mail.org>`); write `<mailto:user@mail.org>` instead.

```
autolink     ::= '<' SCHEME ':' (ANY_CHAR - (SP | '<' | '>'))* '>'

SCHEME            ::= [A-Za-z] [A-Za-z0-9+.-]*
```

Deviation from CommonMark: the target is a full RFC 3987 absolute IRI, so the
scheme carries no 32-character bound and the body must validate as an IRI, not
merely avoid space, `<`, and `>`.

Reduced:

```
<scheme:absolute-uri>
```

## Wiki links

Common core of Obsidian and MediaWiki.

```
wiki-link ::= '[[' target ('#' SECTION)? ('|' TEXT)? ']]'
target    ::= (PAGE_NAME '/')* PAGE_NAME

PAGE_NAME ::= (ANY_CHAR - (NEWLINE | '#' | '|' | '[' | ']' | '/'))+
SECTION   ::= (ANY_CHAR - (NEWLINE | '|' | '[' | ']'))+
TEXT      ::= (ANY_CHAR - (NEWLINE | '[' | ']'))+
```
