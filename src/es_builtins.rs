const BUILTINS: &[(&str, &str)] = &[
    ("abs", "abs"),
    ("all", "todos"),
    ("any", "alguno"),
    ("bool", "logico"),
    ("dict", "dicc"),
    ("enumerate", "enumerar"),
    ("Exception", "Excepcion"),
    ("filter", "filtrar"),
    ("float", "decimal"),
    ("int", "entero"),
    ("input", "leer"),
    ("isinstance", "es_inst"),
    ("issubclass", "es_sub"),
    ("len", "largo"),
    ("list", "lista"),
    ("locals", "locales"),
    ("map", "mapear"),
    ("max", "max"),
    ("min", "min"),
    ("open", "abrir"),
    ("print", "imprimir"),
    ("range", "rango"),
    ("repr", "repr"),
    ("reversed", "invertir"),
    ("round", "redon"),
    ("set", "conjunto"),
    ("sorted", "ordenar"),
    ("str", "cadena"),
    ("sum", "suma"),
    ("tuple", "tupla"),
    ("type", "tipo"),
    ("ValueError", "ErrorValor"),
    ("zip", "unir"),
];

pub fn python_name(name: &str) -> Option<&'static str> {
    BUILTINS
        .iter()
        .find(|(_, translated)| name.eq_ignore_ascii_case(translated))
        .map(|(python, _)| *python)
}

pub fn is_builtin_name(name: &str) -> bool {
    BUILTINS
        .iter()
        .any(|(python, translated)| *python == name || name.eq_ignore_ascii_case(translated))
}
