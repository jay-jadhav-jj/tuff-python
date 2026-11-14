use std::collections::HashMap;
use std::fmt;

const PYTHON_BUILTIN_METHODS: &[&str] = &[
    "__abs__",
    "__add__",
    "__alloc__",
    "__and__",
    "__bool__",
    "__buffer__",
    "__bytes__",
    "__call__",
    "__ceil__",
    "__class_getitem__",
    "__complex__",
    "__contains__",
    "__delattr__",
    "__delete__",
    "__delitem__",
    "__dir__",
    "__divmod__",
    "__enter__",
    "__eq__",
    "__exit__",
    "__float__",
    "__floor__",
    "__floordiv__",
    "__format__",
    "__ge__",
    "__get__",
    "__getattribute__",
    "__getformat__",
    "__getitem__",
    "__getnewargs__",
    "__getstate__",
    "__gt__",
    "__hash__",
    "__iadd__",
    "__iand__",
    "__imul__",
    "__index__",
    "__init__",
    "__init_subclass__",
    "__instancecheck__",
    "__int__",
    "__invert__",
    "__ior__",
    "__isub__",
    "__iter__",
    "__ixor__",
    "__le__",
    "__len__",
    "__length_hint__",
    "__lshift__",
    "__lt__",
    "__mod__",
    "__mul__",
    "__ne__",
    "__neg__",
    "__new__",
    "__next__",
    "__or__",
    "__pos__",
    "__pow__",
    "__prepare__",
    "__radd__",
    "__rand__",
    "__rdivmod__",
    "__reduce__",
    "__reduce_ex__",
    "__release_buffer__",
    "__repr__",
    "__reversed__",
    "__rfloordiv__",
    "__rlshift__",
    "__rmod__",
    "__rmul__",
    "__ror__",
    "__round__",
    "__rpow__",
    "__rrshift__",
    "__rshift__",
    "__rsub__",
    "__rtruediv__",
    "__rxor__",
    "__set__",
    "__set_name__",
    "__setattr__",
    "__setitem__",
    "__setstate__",
    "__sizeof__",
    "__str__",
    "__sub__",
    "__subclasscheck__",
    "__subclasses__",
    "__subclasshook__",
    "__truediv__",
    "__trunc__",
    "__xor__",
    "_from_flags",
    "add",
    "add_note",
    "append",
    "as_integer_ratio",
    "bit_count",
    "bit_length",
    "capitalize",
    "casefold",
    "cast",
    "center",
    "clear",
    "conjugate",
    "copy",
    "count",
    "create_module",
    "decode",
    "deleter",
    "derive",
    "difference",
    "difference_update",
    "discard",
    "encode",
    "endswith",
    "exec_module",
    "expandtabs",
    "extend",
    "find",
    "find_spec",
    "format",
    "format_map",
    "from_bytes",
    "fromhex",
    "fromkeys",
    "get",
    "get_code",
    "get_source",
    "getter",
    "hex",
    "index",
    "indices",
    "insert",
    "intersection",
    "intersection_update",
    "is_integer",
    "is_package",
    "isalnum",
    "isalpha",
    "isascii",
    "isdecimal",
    "isdigit",
    "isdisjoint",
    "isidentifier",
    "islower",
    "isnumeric",
    "isprintable",
    "isspace",
    "issubset",
    "issuperset",
    "istitle",
    "isupper",
    "items",
    "join",
    "keys",
    "ljust",
    "load_module",
    "lower",
    "lstrip",
    "maketrans",
    "mro",
    "partition",
    "pop",
    "popitem",
    "release",
    "remove",
    "removeprefix",
    "removesuffix",
    "replace",
    "reverse",
    "rfind",
    "rindex",
    "rjust",
    "rpartition",
    "rsplit",
    "rstrip",
    "setdefault",
    "setter",
    "sort",
    "split",
    "splitlines",
    "startswith",
    "strip",
    "subgroup",
    "swapcase",
    "symmetric_difference",
    "symmetric_difference_update",
    "title",
    "to_bytes",
    "tobytes",
    "tolist",
    "toreadonly",
    "translate",
    "union",
    "update",
    "upper",
    "values",
    "with_traceback",
    "zfill",
];

const FRENCH_RESERVED_WORDS: &[&str] = &[
    "si",
    "sinon",
    "sinon_si",
    "ou_si",
    "pour",
    "en",
    "tantque",
    "classe",
    "fonction",
    "retournez",
    "interrompez",
    "continuez",
    "et",
    "ou",
    "non",
    "vrai",
    "faux",
    "aucun",
    "selon",
    "cas",
    "importez",
    "depuis",
    "comme",
    "affirme",
    "asynchrone",
    "attendez",
    "effacez",
    "finalement",
    "sauf",
    "avec",
    "rend",
    "essayez",
    "levez",
    "passe",
    "est",
    "imprimez",
    "gamme",
    "longueur",
    "long",
    "locales",
];

#[derive(Default)]
pub struct AliasRegistry {
    aliases: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AliasError {
    ReservedAlias(String),
    ReservedTarget(String),
    ConflictingAlias {
        alias: String,
        existing: String,
        attempted: String,
    },
}

impl fmt::Display for AliasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReservedAlias(alias) => {
                write!(formatter, "alias '{}' is a reserved name", alias)
            }
            Self::ReservedTarget(target) => {
                write!(formatter, "alias target '{}' is a reserved name", target)
            }
            Self::ConflictingAlias {
                alias,
                existing,
                attempted,
            } => write!(
                formatter,
                "alias '{}' already maps to '{}', cannot remap it to '{}'",
                alias, existing, attempted
            ),
        }
    }
}

impl AliasRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks Python keywords, language keywords, and built-in names.
    pub fn is_reserved(word: &str) -> bool {
        matches!(
            word,
            // Python keywords
            "False" | "None" | "True"
            | "and" | "as" | "assert" | "async" | "await"
            | "break" | "class" | "continue"
            | "def" | "del"
            | "elif" | "else" | "except"
            | "finally" | "for" | "from"
            | "global"
            | "if" | "import" | "in" | "is"
            | "lambda"
            | "nonlocal" | "not"
            | "or"
            | "pass"
            | "raise" | "return"
            | "try"
            | "while" | "with"
            | "yield"
            | "match" | "case" | "type"

            // Frenchized Python keywords and translated literals.
            | "si" | "sinon" | "sinon_si"
            | "pour" | "en" | "tantque"
            | "classe" | "fonction" | "retournez"
            | "interrompez" | "continuez"
            | "et" | "ou" | "non"
            | "vrai" | "faux" | "aucun"
            | "selon" | "cas"
            | "importez" | "depuis" | "comme" | "affirme"
            | "asynchrone" | "attendez" | "effacez"
            | "finalement" | "sauf" | "avec" | "rend"
            | "essayez" | "levez" | "passe" | "est"

            // Python built-in constants, functions, types, and exceptions.
            | "__build_class__" | "__debug__" | "__import__"
            | "abs" | "aiter" | "all" | "anext" | "any" | "ascii"
            | "bin" | "bool" | "breakpoint" | "bytearray" | "bytes"
            | "callable" | "chr" | "classmethod" | "compile" | "complex"
            | "delattr" | "dict" | "dir" | "divmod" | "enumerate"
            | "eval" | "exec" | "filter" | "float" | "format" | "frozenset"
            | "getattr" | "globals" | "hasattr" | "hash" | "help" | "hex"
            | "id" | "input" | "int" | "isinstance" | "issubclass" | "iter"
            | "list" | "locals" | "map" | "max" | "memoryview" | "min"
            | "next" | "object" | "oct" | "open" | "ord" | "pow"
            | "print" | "property" | "range" | "repr" | "reversed" | "round"
            | "set" | "setattr" | "slice" | "sorted" | "staticmethod"
            | "str" | "sum" | "super" | "tuple" | "vars" | "zip"
            | "Ellipsis" | "NotImplemented"
            | "BaseException" | "BaseExceptionGroup" | "Exception"
            | "ExceptionGroup" | "GeneratorExit" | "KeyboardInterrupt"
            | "SystemExit" | "ArithmeticError" | "AssertionError"
            | "AttributeError" | "BufferError" | "EOFError" | "ImportError"
            | "ModuleNotFoundError" | "LookupError" | "IndexError" | "KeyError"
            | "MemoryError" | "NameError" | "UnboundLocalError" | "OSError"
            | "BlockingIOError" | "ChildProcessError" | "ConnectionError"
            | "BrokenPipeError" | "ConnectionAbortedError"
            | "ConnectionRefusedError" | "ConnectionResetError"
            | "FileExistsError" | "FileNotFoundError" | "InterruptedError"
            | "IsADirectoryError" | "NotADirectoryError" | "PermissionError"
            | "ProcessLookupError" | "TimeoutError" | "ReferenceError"
            | "RuntimeError" | "NotImplementedError" | "RecursionError"
            | "SyntaxError" | "IndentationError" | "TabError" | "SystemError"
            | "TypeError" | "ValueError" | "UnicodeError"
            | "UnicodeDecodeError" | "UnicodeEncodeError"
            | "UnicodeTranslateError" | "Warning" | "BytesWarning"
            | "DeprecationWarning" | "EncodingWarning" | "FutureWarning"
            | "ImportWarning" | "PendingDeprecationWarning" | "ResourceWarning"
            | "RuntimeWarning" | "SyntaxWarning" | "UnicodeWarning" | "UserWarning"
            | "copyright" | "credits" | "exit" | "license" | "quit"

            // French spellings of translated built-ins.
            | "imprimez" | "gamme" | "longueur" | "long" | "locales"
        ) || PYTHON_BUILTIN_METHODS.contains(&word)
            || crate::builtin_translations::is_builtin_name(word)
            || crate::language_names::is_keyword_name(word)
            || FRENCH_RESERVED_WORDS
                .iter()
                .any(|reserved| word.eq_ignore_ascii_case(reserved))
    }

    /// Registers an alias.
    ///
    /// The first successful registration wins.
    /// Registering the same mapping twice is allowed.
    pub fn register(&mut self, alias: &str, original: &str) -> Result<(), AliasError> {
        if Self::is_reserved(alias) {
            return Err(AliasError::ReservedAlias(alias.to_string()));
        }

        if Self::is_reserved(original) {
            return Err(AliasError::ReservedTarget(original.to_string()));
        }

        if let Some(existing) = self.aliases.get(alias) {
            if existing == original {
                return Ok(());
            }

            return Err(AliasError::ConflictingAlias {
                alias: alias.to_string(),
                existing: existing.clone(),
                attempted: original.to_string(),
            });
        }

        self.aliases.insert(alias.to_string(), original.to_string());

        Ok(())
    }

    /// Resolves an alias to its original identifier.
    pub fn resolve(&self, alias: &str) -> Option<&str> {
        self.aliases.get(alias).map(String::as_str)
    }
}
