# Romanized Marathi (`#!mr`)

This processor accepts ASCII-only Romanized Marathi, without Devanagari or
diacritics. Matching is case-insensitive, and Python spellings remain valid.
It translates defined programming forms; it is not a general Marathi prose
translator.

## Word order and statement forms

Marathi typically uses subject-object-verb (SOV) order. The processor
recognizes these explicit statement templates and rewrites them to Python.
Expressions and quoted text otherwise retain their original order.
Assignments use Python's symbolic operators, for example `number = 0` and
`number += 1`.

| Purpose | Romanized Marathi source | Generated Python |
|---|---|---|
| Conditional | `jar shart tar:` | `if shart:` |
| Alternative condition | `nahitar jar shart tar:` | `elif shart:` |
| Else branch | `nahitar:` | `else:` |
| For loop | `sequence madhye pratyek item:` | `for item in sequence:` |
| While loop | `joparyant shart:` | `while shart:` |
| Function | `naam(params) karya:` | `def naam(params):` |
| Class | `naam varga:` | `class naam:` |
| Return | `value parat` | `return value` |
| Print | `value chhapa` | `print(value)` |
| Exception handler | `Error pakda:` | `except Error:` |
| Import | `math pasun sqrt aayat` | `from math import sqrt` |

`tar` is an optional terminal marker in conditional headers. Python-like
expression order is used inside conditions, calls, and operators.

## Keyword reference

| Python | Romanized Marathi |
|---|---|
| `False` / `None` / `True` | `khota` / `shunya` / `khara` |
| `and` / `or` / `not` | `ani` / `kiwa` / `nahi` |
| `as` / `assert` | `mhanun` / `siddha` |
| `async` / `await` | `async` / `thamba` |
| `break` / `continue` | `baher` / `pudhe` |
| `class` / `def` | `varga` / `karya` |
| `del` | `kadha` |
| `elif` / `else` | `nahitar_jar` / `nahitar` |
| `except` / `finally` | `pakda` / `shevati` |
| `for` / `in` | `pratyek` / `madhye` |
| `from` / `import` | `pasun` / `aayat` |
| `global` / `nonlocal` | `global` / `nonlocal` |
| `if` / `is` | `jar` / `aahe` |
| `lambda` / `pass` | `lambda` / `sodun_dya` |
| `raise` / `return` | `uchla` / `parat` |
| `try` / `while` / `with` | `prayatna` / `joparyant` / `sobat` |
| `yield` | `dya` |
| `match` / `case` / `type` | `julva` / `vakalp` / `type` |

## Built-ins

Common callable built-ins have concise Marathi spellings. `max`, `min`, and
`repr` retain their familiar Python names.

| Python | Romanized Marathi |
|---|---|
| `print` / `input` | `chhapa` / `vacha` |
| `range` / `len` | `shreni` / `lambai` |
| `int` / `float` / `str` | `purnank` / `dashansh` / `majkur` |
| `list` / `dict` / `tuple` / `set` | `yadi` / `shabdkosh` / `jodi` / `samuh` |
| `sum` / `min` / `max` | `berij` / `min` / `max` |
| `locals` | `sthaniya` |
| `sorted` / `reversed` | `lavara` / `ulata` |
| `Exception` / `ValueError` | `Apvad` / `MulyaDosh` |

Other Python built-ins remain available under their Python names. Python
built-in names and translated built-in names are reserved for aliases.
