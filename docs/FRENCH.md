# French (`#!fr`)

This processor accepts unaccented French spellings and case-insensitive
keywords. Python's English spellings remain valid. Python parses and executes
the generated file; this document describes the source constructs that the
transpiler rewrites.

## Statement order

The supported forms follow French clause order where the transpiler has a
specific rewrite. They are syntax forms, not a general French-to-Python
translator.

| Purpose | French source | Generated Python |
|---|---|---|
| Conditional | `si condition alors:` | `if condition:` |
| Alternative condition | `ou si condition alors:` | `elif condition:` |
| Alternative condition (short form) | `sinon_si condition:` | `elif condition:` |
| Else branch | `sinon:` | `else:` |
| For loop | `pour item en sequence:` | `for item in sequence:` |
| While loop | `tantque condition:` | `while condition:` |
| Function | `fonction saluer(nom):` | `def saluer(nom):` |
| Async function | `asynchrone fonction lire():` | `async def lire():` |
| Return | `retournez valeur` | `return valeur` |
| With/as | `avec ressource comme nom:` | `with ressource as nom:` |
| Exception handling | `essayez:` / `sauf Erreur comme e:` | `try:` / `except Erreur as e:` |
| Match | `selon valeur:` / `cas motif:` | `match valeur:` / `case motif:` |

French source and generated Python use the same expression order. The
conditional `alors` marker is optional. Import shorthand supports
`importez sqrt de math`, producing `from math import sqrt`.
French adjectives often follow nouns, but the processor preserves identifiers
and quoted text instead of translating or reordering natural-language prose.

## Keywords and built-ins

Python keyword spellings are Frenchified in [KEYWORD_TRANSLATIONS.md](../KEYWORD_TRANSLATIONS.md).
The table there also lists every callable built-in spelling implemented by
the French processor. Common examples:

| Python | French |
|---|---|
| `if` / `else` | `si` / `sinon` |
| `def` / `return` | `fonction` / `retournez` |
| `True` / `False` / `None` | `vrai` / `faux` / `aucun` |
| `print` / `input` | `imprimez` / `saisir` |
| `range` / `len` | `gamme` / `long` |
| `max` / `min` / `locals` | `max` / `min` / `locales` |
| `map` / `repr` / `vars` | `map` / `repr` / `vars` |

French built-in spellings, like Python built-in names, cannot be used as alias
names or targets.
