# Language-to-Python Keyword Translations

## Command-line usage

The language is inferred from a leading `#!` directive in the source file.
Compile with either command:

```text
tpy c <INPUTFILE> <OUTPUTFILE>
tpy compile <INPUTFILE> <OUTPUTFILE>
```

Put `#!fr`, `#!es`, `#!hi`, `#!mr`, or `#!zh` before source code to select
French, Spanish, Romanized Hindi, Romanized Marathi, or Romanized Mandarin
Chinese. Blank lines and ordinary comment lines may precede the directive.
Language codes are case-insensitive. The transpiler reports an error if the
directive is missing, malformed, or uses an unsupported code.
A directive after source code is not recognized and is reported as a missing
leading directive. Once a valid directive is found, Python handles generated
syntax and runtime errors.

Language-specific guides:

- [French](docs/FRENCH.md)
- [Spanish](docs/SPANISH.md)
- [Romanized Hindi](docs/HINDI_ROMANIZED.md)
- [Romanized Marathi](docs/MARATHI_ROMANIZED.md)
- [Romanized Mandarin Chinese](docs/CHINESE_ROMANIZED.md)

Spanish, Romanized Hindi, Romanized Marathi, and Romanized Chinese spellings
use ASCII letters and underscores only. Python's English spellings also
remain valid.

### Spanish keywords

| Python | Spanish |
|---|---|
| `False` | `falso` |
| `None` | `nada` |
| `True` | `cierto` |
| `and` | `y` |
| `as` | `como` |
| `assert` | `asegura` |
| `async` | `async` |
| `await` | `espera` |
| `break` | `rompe` |
| `class` | `clase` |
| `continue` | `sigue` |
| `def` | `funcion` |
| `del` | `borra` |
| `elif` | `sino_si` / `o si` |
| `else` | `sino` |
| `except` | `excepto` |
| `finally` | `fin` |
| `for` | `para` |
| `from` | `desde` |
| `global` | `global` |
| `if` | `si` |
| `import` | `importar` |
| `in` | `en` |
| `is` | `es` |
| `lambda` | `lambda` |
| `nonlocal` | `nonlocal` |
| `not` | `no` |
| `or` | `o` |
| `pass` | `pasa` |
| `raise` | `lanza` |
| `return` | `retorna` |
| `try` | `intenta` |
| `while` | `mientras` |
| `with` | `con` |
| `yield` | `cede` |
| `match` | `coincide` |
| `case` | `caso` |
| `type` | `type` |

Spanish conditional headers can use `si condicion entonces:` and
`o si condicion entonces:`. The `entonces` marker is optional.

### Romanized Hindi keywords

| Python | Romanized Hindi |
|---|---|
| `False` | `jhooth` |
| `None` | `shunya` |
| `True` | `sach` |
| `and` | `aur` |
| `as` | `jaisa` |
| `assert` | `pakka` |
| `async` | `async` |
| `await` | `ruk` |
| `break` | `todo` |
| `class` | `kaksha` |
| `continue` | `jari` |
| `def` | `karya` |
| `del` | `hatao` |
| `elif` | `warna_agar` / `warna agar` |
| `else` | `warna` |
| `except` | `pakdo` |
| `finally` | `akhir` |
| `for` | `har` |
| `from` | `se` |
| `global` | `global` |
| `if` | `agar` |
| `import` | `lao` |
| `in` | `mein` |
| `is` | `hai` |
| `lambda` | `lambda` |
| `nonlocal` | `nonlocal` |
| `not` | `nahi` |
| `or` | `ya` |
| `pass` | `chhodo` |
| `raise` | `uthao` |
| `return` | `lautao` |
| `try` | `koshish` |
| `while` | `jabtak` |
| `with` | `saath` |
| `yield` | `do` |
| `match` | `milao` |
| `case` | `mamla` |
| `type` | `type` |

Romanized Hindi conditional headers can use `agar shart to:` and
`warna agar shart to:`. The `to` marker is optional.

### Romanized Marathi keywords

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

### Romanized Mandarin Chinese keywords

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

Common built-ins have concise names in every language: `print` is `imprimez`,
`imprimir`, `likho`, `chhapa`, and `dayin`; `max` and `min` keep their short
forms where natural; `locals` becomes `locales`, `locales`, `sthaniya`,
`sthaniya`, and `bendi`. Other built-in functions and types can be called by
their unchanged Python names.

The French keyword table and detailed French statement forms follow.
Language-specific statement templates are documented in the guides above.
These processors translate source syntax; they are not general natural
language prose translators.

## Python keywords

| Python spelling | French spelling | Notes |
|---|---|---|
| `False` | `faux` | Boolean literal |
| `None` | `aucun` | Null literal |
| `True` | `vrai` | Boolean literal |
| `and` | `et` | |
| `as` | `comme` | |
| `assert` | `affirme` | |
| `async` | `asynchrone` / `async` | The established short form `async` is also accepted directly |
| `await` | `attendez` | |
| `break` | `interrompez` | |
| `class` | `classe` | |
| `continue` | `continuez` | |
| `def` | `fonction` | |
| `del` | `effacez` | |
| `elif` | `ou si` / `sinon_si` | `ou si` is natural French; `sinon_si` remains supported |
| `else` | `sinon` | |
| `except` | `sauf` | |
| `finally` | `finalement` | |
| `for` | `pour` | |
| `from` | `depuis` | Also supports import shorthand below |
| `global` | `global` | Same spelling |
| `if` | `si` | |
| `import` | `importez` | |
| `in` | `en` | |
| `is` | `est` | |
| `lambda` | `lambda` | Same spelling |
| `nonlocal` | `nonlocal` | Same spelling |
| `not` | `non` | |
| `or` | `ou` | |
| `pass` | `passe` | |
| `raise` | `levez` | |
| `return` | `retournez` | |
| `try` | `essayez` | |
| `while` | `tantque` | |
| `with` | `avec` | |
| `yield` | `rend` | |
| `match` | `selon` | Soft keyword |
| `case` | `cas` | Soft keyword |
| `type` | `type` | Soft keyword; retained as-is |

`True`, `False`, and `None` are literals rather than Python keywords, but are
included because the transpiler translates them.

## Built-in callables and constructors

These are unaccented French spellings for callable names from Python's
`builtins` module, including functions, constructors/types, exceptions,
warnings, and interactive helpers. Python's English spelling remains valid.
Names that are implementation- or version-specific are included when exposed
by the Python runtime.

| Python name | French spelling |
|---|---|
| `ArithmeticError` | `erreur_arithmetique` |
| `AssertionError` | `erreur_assertion` |
| `AttributeError` | `erreur_attribut` |
| `BaseException` | `exception_base` |
| `BaseExceptionGroup` | `groupe_exceptions_base` |
| `BlockingIOError` | `erreur_e_s_bloquante` |
| `BrokenPipeError` | `erreur_tuyau_casse` |
| `BufferError` | `erreur_tampon` |
| `BytesWarning` | `avertissement_octets` |
| `ChildProcessError` | `erreur_processus_enfant` |
| `ConnectionAbortedError` | `erreur_connexion_abandonnee` |
| `ConnectionError` | `erreur_connexion` |
| `ConnectionRefusedError` | `erreur_connexion_refusee` |
| `ConnectionResetError` | `erreur_connexion_reinitialisee` |
| `DeprecationWarning` | `avertissement_obsolescence` |
| `EOFError` | `erreur_fin_fichier` |
| `EncodingWarning` | `avertissement_encodage` |
| `EnvironmentError` | `erreur_environnement` |
| `Exception` | `exception` |
| `ExceptionGroup` | `groupe_exceptions` |
| `FileExistsError` | `erreur_fichier_existant` |
| `FileNotFoundError` | `erreur_fichier_introuvable` |
| `FloatingPointError` | `erreur_flottant` |
| `FutureWarning` | `avertissement_futur` |
| `GeneratorExit` | `sortie_generateur` |
| `IOError` | `erreur_entree_sortie` |
| `ImportError` | `erreur_importation` |
| `ImportWarning` | `avertissement_importation` |
| `IndentationError` | `erreur_indentation` |
| `IndexError` | `erreur_indice` |
| `InterruptedError` | `erreur_interruption` |
| `IsADirectoryError` | `erreur_est_repertoire` |
| `KeyError` | `erreur_cle` |
| `KeyboardInterrupt` | `interruption_clavier` |
| `LookupError` | `erreur_recherche` |
| `MemoryError` | `erreur_memoire` |
| `ModuleNotFoundError` | `erreur_module_introuvable` |
| `NameError` | `erreur_nom` |
| `NotADirectoryError` | `erreur_pas_repertoire` |
| `NotImplementedError` | `erreur_non_implementee` |
| `OSError` | `erreur_systeme` |
| `OverflowError` | `erreur_depassement` |
| `PendingDeprecationWarning` | `avertissement_obsolescence_future` |
| `PermissionError` | `erreur_permission` |
| `ProcessLookupError` | `erreur_recherche_processus` |
| `PythonFinalizationError` | `erreur_finalisation_python` |
| `RecursionError` | `erreur_recursion` |
| `ReferenceError` | `erreur_reference` |
| `ResourceWarning` | `avertissement_ressource` |
| `RuntimeError` | `erreur_execution` |
| `RuntimeWarning` | `avertissement_execution` |
| `StopAsyncIteration` | `fin_iteration_asynchrone` |
| `StopIteration` | `fin_iteration` |
| `SyntaxError` | `erreur_syntaxe` |
| `SyntaxWarning` | `avertissement_syntaxe` |
| `SystemError` | `erreur_systeme_interne` |
| `SystemExit` | `sortie_systeme` |
| `TabError` | `erreur_tabulation` |
| `TimeoutError` | `erreur_delai` |
| `TypeError` | `erreur_type` |
| `UnboundLocalError` | `erreur_locale_non_liee` |
| `UnicodeDecodeError` | `erreur_decodage_unicode` |
| `UnicodeEncodeError` | `erreur_encodage_unicode` |
| `UnicodeError` | `erreur_unicode` |
| `UnicodeTranslateError` | `erreur_traduction_unicode` |
| `UnicodeWarning` | `avertissement_unicode` |
| `UserWarning` | `avertissement_utilisateur` |
| `ValueError` | `erreur_valeur` |
| `Warning` | `avertissement` |
| `WindowsError` | `erreur_windows` |
| `ZeroDivisionError` | `erreur_division_zero` |
| `_IncompleteInputError` | `erreur_entree_incomplete` |
| `__build_class__` | `construire_classe` |
| `__import__` | `importer_module` |
| `__loader__` | `chargeur` |
| `abs` | `valeur_absolue` |
| `aiter` | `iterateur_asynchrone` |
| `all` | `tous` |
| `anext` | `suivant_asynchrone` |
| `any` | `au_moins_un` |
| `ascii` | `representation_ascii` |
| `bin` | `binaire` |
| `bool` | `booleen` |
| `breakpoint` | `point_arret` |
| `bytearray` | `tableau_octets` |
| `bytes` | `octets` |
| `callable` | `est_appelable` |
| `chr` | `caractere` |
| `classmethod` | `methode_classe` |
| `compile` | `compiler` |
| `complex` | `complexe` |
| `copyright` | `droit_auteur` |
| `credits` | `credits` |
| `delattr` | `supprimer_attribut` |
| `dict` | `dictionnaire` |
| `dir` | `noms_attributs` |
| `divmod` | `division_quotient_reste` |
| `enumerate` | `enumerer` |
| `eval` | `evaluer` |
| `exec` | `executer` |
| `exit` | `quitter` |
| `filter` | `filtrer` |
| `float` | `flottant` |
| `format` | `formater` |
| `frozenset` | `ensemble_immuable` |
| `getattr` | `obtenir_attribut` |
| `globals` | `variables_globales` |
| `hasattr` | `possede_attribut` |
| `hash` | `hachage` |
| `help` | `aide` |
| `hex` | `hexadecimal` |
| `id` | `identifiant` |
| `input` | `saisir` |
| `int` | `entier` |
| `isinstance` | `est_instance` |
| `issubclass` | `est_sous_classe` |
| `iter` | `iterateur` |
| `len` | `long` |
| `license` | `licence` |
| `list` | `liste` |
| `locals` | `locales` |
| `map` | `map` |
| `max` | `max` |
| `memoryview` | `vue_memoire` |
| `min` | `min` |
| `next` | `suivant` |
| `object` | `objet` |
| `oct` | `octal` |
| `open` | `ouvrir` |
| `ord` | `code_caractere` |
| `pow` | `puissance` |
| `print` | `imprimez` |
| `property` | `propriete` |
| `quit` | `sortir` |
| `range` | `gamme` |
| `repr` | `repr` |
| `reversed` | `inverse` |
| `round` | `arrondir` |
| `set` | `ensemble` |
| `setattr` | `definir_attribut` |
| `slice` | `tranche` |
| `sorted` | `trier` |
| `staticmethod` | `methode_statique` |
| `str` | `chaine` |
| `sum` | `somme` |
| `super` | `super_classe` |
| `tuple` | `uplet` |
| `type` | `type` |
| `vars` | `vars` |
| `zip` | `assembler` |

## What “soft keyword” means

A hard keyword such as `si` (Python `if`) is reserved as a keyword wherever it
appears as an identifier. A soft keyword is contextual: Python treats it as a
keyword only in particular grammar positions, and it can still be an ordinary
identifier elsewhere. `match` and `case` are soft keywords in Python's
pattern-matching syntax; `type` is soft in Python's type-alias syntax.

French keyword spellings are recognized without regard to capitalization: for
example, `Vrai`, `FAUX`, and `Aucun` emit the canonically cased Python literals
`True`, `False`, and `None`. The accepted short form `Async` also emits
canonical `async`. Other recognized French keyword spellings are
case-insensitive.

The transpiler replaces `selon` with `match` and `cas` with `case` wherever
those words occur as tokens. It does not decide whether they are in a valid
soft-keyword context; Python parses the generated code and reports syntax
errors. `type` is already spelled the same in French and is passed through.

## Statement word order

The transpiler supports selected French statement forms, not arbitrary
French-language prose. It recognizes the `alors` marker on conditional
headers and the natural `ou si` continuation at the start of a line. Other
parts of a statement retain their order and must use the shown code structure.
The examples show accepted source and generated Python order.

| Purpose | French source order | Python output order |
|---|---|---|
| Conditional | `si condition alors:` (or `si condition:`) | `if condition:` |
| Alternate condition | `ou si condition alors:` | `elif condition:` |
| Alternate condition (compatibility spelling) | `sinon_si condition:` | `elif condition:` |
| Alternative branch | `sinon:` | `else:` |
| For loop | `pour element en sequence:` | `for element in sequence:` |
| While loop | `tantque condition:` | `while condition:` |
| Function | `fonction nom(parametres):` | `def nom(parametres):` |
| Async function | `asynchrone fonction nom():` or `async fonction nom():` | `async def nom():` |
| Class | `classe Nom:` | `class Nom:` |
| Return | `retournez valeur` | `return valeur` |
| With/as | `avec expression comme nom:` | `with expression as nom:` |
| Try/except | `essayez:` then `sauf Exception comme erreur:` | `try:` then `except Exception as erreur:` |
| Finally | `finalement:` | `finally:` |
| Match/case | `selon valeur:` then `cas motif:` | `match valeur:` then `case motif:` |
| Boolean expression | `non a et b ou c` | `not a and b or c` |

Indentation, punctuation, and the order of the rest of each statement are
preserved. These examples show keyword substitutions, not a promise that every
combination is valid Python; Python remains responsible for syntax validation.

Example:

```text
si score >= 90 alors:
    imprimez("excellent")
ou si score >= 60 alors:
    imprimez("reussi")
sinon:
    imprimez("a refaire")
```

```python
if score >= 90:
    print("excellent")
elif score >= 60:
    print("reussi")
else:
    print("a refaire")
```

### Imports

Normal import order is translated directly:

```python
importez math
depuis math importez sqrt
```

```python
import math
from math import sqrt
```

There is also a special shorthand handled by the transpiler:

```python
importez sqrt de math
```

```python
from math import sqrt
```

Only `importez ... de ...` triggers this reordering. `de` is not a general
translation for `from`, and other French word orders are not parsed or moved.

## Aliases

Aliases use `french_name{compiled_name}`, for example:

```python
fonction saluer{greet}(nom):
    retournez "Bonjour " + nom
```

The alias maps the non-reserved source identifier `saluer` to `greet` in its
declaration and subsequent references.

- Python keywords and their French spellings are reserved on both sides of an
  alias. They cannot be alias names or compiled targets.
- French keyword reservation is case-insensitive, so forms such as `Vrai` are
  still reserved in alias declarations.
- Python built-in names are reserved on both sides too. This includes
  built-in functions (`print`, `input`, `len`), types/classes (`list`, `dict`,
  `int`, `Exception`), constants (`Ellipsis`, `NotImplemented`), and
  exceptions. Built-in type method names such as `append`, `upper`, and
  `items` are reserved too. Their translated French spellings (`imprimez`,
  `gamme`, `long`) are reserved as well.
- Reservation applies only to alias declarations and targets. Ordinary
  built-in member access such as `items.append(value)` remains valid and is
  passed through unchanged.
- Repeating the same alias mapping is allowed and does not change it.
- A conflicting second mapping is rejected; it cannot overwrite the first.
  The transpiler reports an alias diagnostic and still emits generated code.
- Non-alias Python syntax errors are passed through for Python to diagnose.
