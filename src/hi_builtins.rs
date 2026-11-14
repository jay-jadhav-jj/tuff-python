const BUILTINS: &[(&str, &str)] = &[
    ("abs", "poorn"),
    ("all", "sab"),
    ("any", "koi"),
    ("bool", "sahi"),
    ("dict", "naksha"),
    ("enumerate", "gino"),
    ("Exception", "Apvad"),
    ("filter", "chhano"),
    ("float", "dashamlav"),
    ("int", "ank"),
    ("input", "lo"),
    ("isinstance", "kism"),
    ("issubclass", "upkism"),
    ("len", "lambai"),
    ("list", "suchi"),
    ("locals", "sthaniya"),
    ("map", "lagao"),
    ("max", "max"),
    ("min", "min"),
    ("open", "kholo"),
    ("print", "likho"),
    ("range", "daayra"),
    ("repr", "roop"),
    ("reversed", "ulta"),
    ("round", "gol"),
    ("set", "samuh"),
    ("sorted", "sajao"),
    ("str", "shabd"),
    ("sum", "jod"),
    ("tuple", "joda"),
    ("type", "prakaar"),
    ("ValueError", "MoolDosh"),
    ("zip", "jodo"),
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
