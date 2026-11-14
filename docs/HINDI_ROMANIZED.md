# Romanized Hindi (`#!hi`)

This language uses Romanized Hindi, not Devanagari. Keywords are ASCII-only
and case-insensitive. Python's English spellings also remain valid.

## Word order

Hindi is generally subject-object-verb (SOV), unlike Python's code grammar.
The processor supports explicit Hindi-ordered forms for common statements
instead of blindly substituting words in those forms. It does not reorder
arbitrary prose or expressions. Hindi adjectives normally precede nouns, but
quoted text and identifiers are preserved; the transpiler does not inflect or
reorder natural-language descriptions. Assignments use Python's symbolic
operators, for example `number = 0` and `number += 1`.

| Purpose | Romanized Hindi source | Generated Python |
|---|---|---|
| Conditional | `agar shart to:` | `if shart:` |
| Alternative condition | `warna agar shart to:` or `warna_agar shart:` | `elif shart:` |
| Else branch | `warna:` | `else:` |
| For loop | `sequence mein har item:` | `for item in sequence:` |
| While loop | `jabtak shart:` | `while shart:` |
| Function | `naam(params) karya:` | `def naam(params):` |
| Class | `naam kaksha:` | `class naam:` |
| Return | `value lautao` | `return value` |
| Print statement | `value likho` | `print(value)` |
| Exception handler | `Error pakdo:` | `except Error:` |
| Import | `module se naam lao` | `from module import naam` |

The `to` marker after an `agar` condition is optional. The forms in the table
are the supported grammatical templates; use Python-like expression order
inside conditions, calls, and operators.

## Keyword reference

| Python | Romanized Hindi |
|---|---|
| `False` / `None` / `True` | `jhooth` / `shunya` / `sach` |
| `and` / `or` / `not` | `aur` / `ya` / `nahi` |
| `as` / `assert` | `jaisa` / `pakka` |
| `async` / `await` | `async` / `ruk` |
| `break` / `continue` | `todo` / `jari` |
| `class` / `def` | `kaksha` / `karya` |
| `del` | `hatao` |
| `elif` / `else` | `warna_agar` / `warna` |
| `except` / `finally` | `pakdo` / `akhir` |
| `for` / `in` | `har` / `mein` |
| `from` / `import` | `se` / `lao` |
| `global` / `nonlocal` | `global` / `nonlocal` |
| `if` / `is` | `agar` / `hai` |
| `lambda` / `pass` | `lambda` / `chhodo` |
| `raise` / `return` | `uthao` / `lautao` |
| `try` / `while` / `with` | `koshish` / `jabtak` / `saath` |
| `yield` | `do` |
| `match` / `case` / `type` | `milao` / `mamla` / `type` |

## Built-ins

Common callable built-ins use short ASCII Romanized Hindi names:

| Python | Romanized Hindi |
|---|---|
| `print` / `input` | `likho` / `lo` |
| `range` / `len` | `daayra` / `lambai` |
| `int` / `float` / `str` | `ank` / `dashamlav` / `shabd` |
| `list` / `dict` / `tuple` / `set` | `suchi` / `naksha` / `joda` / `samuh` |
| `sum` / `min` / `max` | `jod` / `min` / `max` |
| `locals` | `sthaniya` |
| `repr` | `roop` |
| `sorted` / `reversed` | `sajao` / `ulta` |
| `Exception` / `ValueError` | `Apvad` / `MoolDosh` |

Other Python built-in callables can be used under their unchanged Python names.
Both translated and Python built-in names are reserved for aliases.
