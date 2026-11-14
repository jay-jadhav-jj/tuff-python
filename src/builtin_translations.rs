use crate::language_names::Language;

pub fn python_name(language: Language, name: &str) -> Option<&'static str> {
    match language {
        Language::French => crate::fr_builtins::python_name(name),
        Language::Spanish => crate::es_builtins::python_name(name),
        Language::Hindi => crate::hi_builtins::python_name(name),
        Language::Marathi => crate::mr_builtins::python_name(name),
        Language::Chinese => crate::zh_builtins::python_name(name),
    }
}

pub fn is_builtin_name(name: &str) -> bool {
    crate::fr_builtins::FRENCH_BUILTINS
        .iter()
        .any(|(python, translated)| *python == name || name.eq_ignore_ascii_case(translated))
        || crate::es_builtins::is_builtin_name(name)
        || crate::hi_builtins::is_builtin_name(name)
        || crate::mr_builtins::is_builtin_name(name)
        || crate::zh_builtins::is_builtin_name(name)
}
