# Romanized Mandarin Chinese (`#!zh`)

This processor accepts Mandarin Chinese in plain Pinyin spelling without tone
marks, using ASCII letters and underscores. Matching is case-insensitive, and
Python spellings remain valid. It translates programming syntax rather than
arbitrary Chinese prose.

## Word order and statement forms

Mandarin is generally subject-verb-object (SVO), so most translated
statements keep Python's familiar expression order. The `for` loop and
from-import forms below follow readable Pinyin word order and are reordered
explicitly.

| Purpose | Romanized Chinese source | Generated Python |
|---|---|---|
| Conditional | `ruguo shart na:` | `if shart:` |
| Alternative condition | `fouze ruguo shart na:` | `elif shart:` |
| Else branch | `fouze:` | `else:` |
| For loop | `mei_ge item zai sequence:` | `for item in sequence:` |
| While loop | `dang shart:` | `while shart:` |
| Function | `dingyi naam(params):` | `def naam(params):` |
| Class | `lei Naam:` | `class Naam:` |
| Return | `fanhui value` | `return value` |
| Print | `dayin(value)` | `print(value)` |
| Exception handling | `changshi:` / `chuwai Error:` | `try:` / `except Error:` |
| From import | `cong math daoru sqrt` | `from math import sqrt` |

The optional `na` marker after an `ruguo` condition is removed from the
generated header. Operators and expressions keep Python order.

## Keyword reference

| Python | Romanized Chinese (Pinyin) |
|---|---|
| `False` / `None` / `True` | `jia` / `wu` / `zhen` |
| `and` / `or` / `not` | `he` / `huo` / `bu` |
| `as` / `assert` | `zuowei` / `queren` |
| `async` / `await` | `async` / `dengdai` |
| `break` / `continue` | `tupo` / `jixu` |
| `class` / `def` | `lei` / `dingyi` |
| `del` | `shanchu` |
| `elif` / `else` | `fouze_ruguo` / `fouze` |
| `except` / `finally` | `chuwai` / `zuihou` |
| `for` / `in` | `mei_ge` / `zai` |
| `from` / `import` | `cong` / `daoru` |
| `global` / `nonlocal` | `global` / `nonlocal` |
| `if` / `is` | `ruguo` / `shi` |
| `lambda` / `pass` | `lambda` / `tiaoguo` |
| `raise` / `return` | `paichu` / `fanhui` |
| `try` / `while` / `with` | `changshi` / `dang` / `yong` |
| `yield` | `shuchu` |
| `match` / `case` / `type` | `pipei` / `qingkuang` / `type` |

## Built-ins

Common callable built-ins use recognizable, compact Pinyin names. `max` and
`min` keep their established Python spellings.

| Python | Romanized Chinese |
|---|---|
| `print` / `input` | `dayin` / `shuru` |
| `range` / `len` | `fanwei` / `changdu` |
| `int` / `float` / `str` | `zhengshu` / `xiaoshu` / `zifu` |
| `list` / `dict` / `tuple` / `set` | `liebiao` / `zidian` / `yuanzu` / `jihe` |
| `sum` / `min` / `max` | `zonghe` / `min` / `max` |
| `locals` | `bendi` |
| `sorted` / `reversed` | `paixu` / `daoxu` |
| `Exception` / `ValueError` | `Yichang` / `ZhixingCuowu` |

Other Python built-ins remain available under their Python names. Python
built-in names and translated built-in names are reserved for aliases.
