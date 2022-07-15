"""cntlang entry point"""

__version__ = "0.1.0"


class Colors:  # You may need to change color settings
    RED = "\033[31m"
    ENDC = "\033[m"
    GREEN = "\033[32m"
    YELLOW = "\033[33m"
    BLUE = "\033[34m"

    def colored(self, string, color):
        return f"{color}{string}{Colors.ENDC}"


ext2name = {
    "sh": "Bash/Fish",
    "fish": "Bash/Fish",
    "c": "C99/C++",
    "js": "JavaScript",
    "ts": "JavaScript",
    "asm": "Assembly",
    "nasm": "Assembly",
    "scm": "Scheme",
    "rkt": "Scheme",
    "hs": "Haskell",
    "jl": "Julia",
    "go": "Golang",
    "lua": "Lua",
    "py": "Python",
    "rs": "Rust",
    "r": "RLang",
    "sv": "Verilog",
    "php": "PHP",
}

igndir = {
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
}

ignfile = {
    "setupTypeScript",
}
