# Multilingual Transpiler Test Results

**Overall: PASS**

Generated Python files and temporary exhaustive-audit inputs were removed.
The 45 checked-in `.tpy` examples (9 per language) are retained in the
language test folders.

## Static checks

| Check | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo test --all-targets` | PASS — 19 CLI integration tests; 0 unit tests |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `cargo build --bin tpy` | PASS — no warnings |
| Warning suppressions | None present |

## End-to-end tests

Each valid case was compiled with the `tpy c <input> <output>` command and run
with Python, or checked for expected generated code/diagnostics.

| Test | Result | Verified behavior |
|---|---|---|
| All mapped French keywords | PASS | Canonical Python spelling and casing emitted |
| Spanish keywords and built-ins | PASS | Spanish source transpiles and executes |
| Romanized Hindi keywords and built-ins | PASS | ASCII Hindi source transpiles and executes |
| Romanized Marathi keywords and built-ins | PASS | ASCII Marathi source transpiles and executes |
| Romanized Chinese keywords and built-ins | PASS | Pinyin source transpiles and executes |
| Hindi SOV statement forms | PASS | Reorders function, return, print, assignment, loop, class, exception, and import forms |
| Marathi SOV statement forms | PASS | Reorders function, return, print, assignment, loop, class, exception, and import forms |
| Chinese statement ordering | PASS | Reorders `mei_ge ... zai ...` loops and `cong ... daoru ...` imports |
| Localized import ordering | PASS | Spanish, Hindi, Marathi, and Chinese import forms become Python imports |
| Localized built-in alias reservation | PASS | Translated built-ins cannot be used as alias targets in Spanish, Marathi, or Chinese |
| Concise built-in spellings | PASS | `max`/`min` keep short names; `locals` uses `locales`/`sthaniya` |
| Short built-in execution | PASS | `max`, `min`, `len`, and localized `locals` names execute in all five languages |
| Symbolic assignments | PASS | `=`, `+=`, `*=`, and `-=` are preserved verbatim and execute in all five languages |
| Language examples | PASS | All 45 examples (9 per language) compile and produce expected matching output |
| Previous exhaustive builtin/alias audit | PASS | Existing French, Spanish, and Hindi audit: all localized entries transpiled; 432 alias-name/target reservation cases rejected |
| Python built-in callable coverage | PASS | French code and documentation match all 150 callable names exposed by the installed Python runtime |
| Previous per-language built-in translation audit | PASS | Existing audit covered 216 entries (150 French, 33 Spanish, 33 Hindi); new languages are verified by the current executable examples and smoke tests |
| Built-in alias reservation | PASS | Python built-in names and each language's localized names are reserved as alias names and targets |
| Built-in execution | PASS | Three scripts exercised translated functions, constructors, and types |
| `Vrai`, `Faux`, `Aucun` | PASS | Emitted `True`, `False`, `None` |
| `async` shorthand | PASS | `async` and `asynchrone` both emit canonical `async` |
| Conditional with `alors` | PASS | `si … alors:` emitted `if …:` |
| Natural alternative condition | PASS | `ou si … alors:` emitted `elif …:` |
| Capitalized `OU SI` | PASS | Case-insensitive conditional recognition |
| Existing `sinon_si` spelling | PASS | Still emitted `elif` |
| Conditional branch execution | PASS | Correct branch output: `reussi` |
| Boolean `ou` expression | PASS | Emitted `or` and executed correctly |
| Function alias and return | PASS | Generated code executed and printed `hi Ada` |
| Import reordering | PASS | `importez sqrt de math` emitted a working `from` import |
| Match/case | PASS | Generated code executed and printed `two` |
| Keyword text in strings/comments | PASS | Not translated as code |
| Capitalized reserved alias | PASS | Alias diagnostic emitted; code retained |
| Invalid generated Python (Python runtime) | PASS | Transpiler emitted code; Python reported `SyntaxError` |
| Language directive checks | PASS | Language codes are case-insensitive; missing, malformed, unsupported, or late directives produce specific errors |
| CLI argument errors | PASS | Invalid argument count shows usage |
| Missing input path | PASS | Reports the file-not-found cause without unrelated language-directive advice |
| Missing language directive | PASS | Clear diagnostic; no output file written |
| Unsupported language code | PASS | Names invalid code and supported `fr`, `es`, `hi`, `mr`, `zh`; no output file written |
| Malformed language directive | PASS | Reports missing language code; no output file written |
| Directive after source code | PASS | Reports missing leading directive |
| Empty source | PASS | Reports missing language directive |
| Output write failure | PASS | Reports the failing output path |
| Invalid generated Python (CLI) | PASS | Transpiler still writes output for Python to diagnose |

## Word-order scope

The processors implement the language-specific statement templates documented
in the language guides. Hindi and Marathi SOV forms for function declarations,
return/print statements, assignments, loops, and imports are reordered
explicitly. Chinese uses SVO expression order with explicit loop and import
reordering. Arbitrary natural-language sentences and adjective agreement are
not translated; expression order and quoted text remain unchanged.
