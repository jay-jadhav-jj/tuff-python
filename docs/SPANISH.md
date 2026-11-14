# Spanish (`#!es`)

Use unaccented Spanish keywords. Matching is case-insensitive. Python's
English spellings remain valid. All source spellings use ASCII letters and
underscores only.

## Statement order and grammar

Spanish normally uses subject-verb-object order. The supported programming
forms put the action and its operands in readable Spanish order; expressions
and function argument lists otherwise retain Python's familiar structure.
Spanish commonly places adjectives after nouns. Identifiers, expressions,
and quoted text are preserved rather than grammatically rewritten. This is a
syntax-aware transpiler, not a general Spanish prose translator.

| Purpose | Spanish source | Generated Python |
|---|---|---|
| Conditional | `si condicion entonces:` | `if condicion:` |
| Alternative condition | `o si condicion entonces:` or `sino_si condicion:` | `elif condicion:` |
| Else branch | `sino:` | `else:` |
| For loop | `para item en secuencia:` | `for item in secuencia:` |
| While loop | `mientras condicion:` | `while condicion:` |
| Function | `funcion saludar(nombre):` | `def saludar(nombre):` |
| Return | `retorna valor` | `return valor` |
| With/as | `con recurso como nombre:` | `with recurso as nombre:` |
| Exception handling | `intenta:` / `excepto Error como e:` | `try:` / `except Error as e:` |
| Import shorthand | `importar sqrt de math` | `from math import sqrt` |

`entonces` is optional in conditional headers. Conditions, assignments, and
expressions keep their Python operator order; the transpiler does not reorder
arbitrary prose.

## Keyword reference

| Python | Spanish |
|---|---|
| `False` / `None` / `True` | `falso` / `nada` / `cierto` |
| `and` / `or` / `not` | `y` / `o` / `no` |
| `as` / `assert` | `como` / `asegura` |
| `async` / `await` | `async` / `espera` |
| `break` / `continue` | `rompe` / `sigue` |
| `class` / `def` | `clase` / `funcion` |
| `del` | `borra` |
| `elif` / `else` | `sino_si` / `sino` |
| `except` / `finally` | `excepto` / `fin` |
| `for` / `in` | `para` / `en` |
| `from` / `import` | `desde` / `importar` |
| `global` / `nonlocal` | `global` / `nonlocal` |
| `if` / `is` | `si` / `es` |
| `lambda` / `pass` | `lambda` / `pasa` |
| `raise` / `return` | `lanza` / `retorna` |
| `try` / `while` / `with` | `intenta` / `mientras` / `con` |
| `yield` | `cede` |
| `match` / `case` / `type` | `coincide` / `caso` / `type` |

## Built-ins

Common callable built-ins have short Spanish names:

| Python | Spanish |
|---|---|
| `print` / `input` | `imprimir` / `leer` |
| `range` / `len` | `rango` / `largo` |
| `int` / `float` / `str` | `entero` / `decimal` / `cadena` |
| `list` / `dict` / `tuple` / `set` | `lista` / `dicc` / `tupla` / `conjunto` |
| `sum` / `min` / `max` | `suma` / `min` / `max` |
| `locals` | `locales` |
| `repr` | `repr` |
| `sorted` / `reversed` | `ordenar` / `invertir` |
| `Exception` / `ValueError` | `Excepcion` / `ErrorValor` |

Other Python built-in callables can be used under their unchanged Python names.
Both translated and Python built-in names are reserved for aliases.
