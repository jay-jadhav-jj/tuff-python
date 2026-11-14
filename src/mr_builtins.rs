const BUILTINS: &[(&str, &str)] = &[
    ("abs", "nirpeksha"),
    ("all", "sarva"),
    ("any", "kahi"),
    ("bool", "khare_khote"),
    ("dict", "shabdkosh"),
    ("enumerate", "mojani"),
    ("Exception", "Apvad"),
    ("filter", "galani"),
    ("float", "dashansh"),
    ("int", "purnank"),
    ("input", "vacha"),
    ("isinstance", "prakar_ahe"),
    ("issubclass", "upprakar_ahe"),
    ("len", "lambai"),
    ("list", "yadi"),
    ("locals", "sthaniya"),
    ("map", "nakasha"),
    ("max", "max"),
    ("min", "min"),
    ("open", "ugha"),
    ("print", "chhapa"),
    ("range", "shreni"),
    ("repr", "repr"),
    ("reversed", "ulata"),
    ("round", "gol"),
    ("set", "samuh"),
    ("sorted", "lavara"),
    ("str", "majkur"),
    ("sum", "berij"),
    ("tuple", "jodi"),
    ("type", "prakar"),
    ("ValueError", "MulyaDosh"),
    ("zip", "joda"),
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
