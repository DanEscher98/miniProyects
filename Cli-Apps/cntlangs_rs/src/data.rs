use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// EXT2NAME is a hashmap that stores the extension of a file and the corresponding language
    pub static ref EXT2NAME: HashMap<&'static str, &'static str> = {
        let data: Vec<(&str, &str)> = vec![
            ("sh", "Bash/Fish"),
            ("fish", "Bash/Fish"),
            ("js", "JavaScript"),
            ("ts", "JavaScript"),
            ("asm", "Assembly"),
            ("nasm", "Assembly"),
            ("scm", "Scheme"),
            ("rkt", "Scheme"),
            ("hs", "Haskell"),
            ("jl", "Julia"),
            ("go", "Golang"),
            ("lua", "Lua"),
            ("py", "Python"),
            ("rs", "Rust"),
            ("r", "RLang"),
            ("sv", "Verilog"),
            ("php", "PHP"),
        ];
        let mut hashmap = HashMap::new();
        for &(ext, name) in data.iter() {
            hashmap.insert(ext, name);
        }
        hashmap
    };
    /// IGNDIR is a vector that contains directory names that should ignored
    pub static ref IGNDIR: Vec<&'static str> = vec![
        "bin",
        "build",
        "config",
        "env",
        "deps",
        "dev",
        "dist",
        "node_modules",
        "public",
        "target",
        "venv",
        ".venv",
        "__pycache__",
        "vendor",
        "storage",
        "python-packages",
        "py-support",
        "Experiments",
        ".ccls-cache",
        ".ipynb_checkpoints",
        ".hidden",
        ".cabal",
        ".stack-work",
        ".pyenv",
        ".local",
        ".git"
    ];
    /// IGNDIR is a vector that contains names of file that should be ignored
    pub static ref IGNFILE: Vec<&'static str> = vec!["setupTypeScript"];
}
