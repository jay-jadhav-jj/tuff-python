const BUILTINS: &[(&str, &str)] = &[
    ("abs", "juedui"),
    ("all", "quanbu"),
    ("any", "renhe"),
    ("bool", "buer"),
    ("dict", "zidian"),
    ("enumerate", "bianhao"),
    ("Exception", "Yichang"),
    ("filter", "shaixuan"),
    ("float", "xiaoshu"),
    ("int", "zhengshu"),
    ("input", "shuru"),
    ("isinstance", "shifou_shili"),
    ("issubclass", "shifou_zilei"),
    ("len", "changdu"),
    ("list", "liebiao"),
    ("locals", "bendi"),
    ("map", "yingyong"),
    ("max", "max"),
    ("min", "min"),
    ("open", "dakai"),
    ("print", "dayin"),
    ("range", "fanwei"),
    ("repr", "biaoshi"),
    ("reversed", "daoxu"),
    ("round", "siyuanshewu"),
    ("set", "jihe"),
    ("sorted", "paixu"),
    ("str", "zifu"),
    ("sum", "zonghe"),
    ("tuple", "yuanzu"),
    ("type", "leixing"),
    ("ValueError", "ZhixingCuowu"),
    ("zip", "dapei"),
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
