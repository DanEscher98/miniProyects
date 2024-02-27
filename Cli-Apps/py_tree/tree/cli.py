import argparse
import os
from pathlib import Path
from argparse import Namespace


def path_isdir(dir_path: str) -> Path:
    if not os.path.exists(dir_path):
        raise argparse.ArgumentTypeError(
                f"Path '{dir_path}' does not exist.")
    dir = Path(dir_path)
    if not dir.is_dir():
        raise argparse.ArgumentTypeError(
                f"'{dir_path}' isn't a directory.")
    return dir


def parse_args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog="tree",
        usage="%(prog)s [options] DIR",
        description="Shows contents of a directory in tree form",
    )
    # parser.version = f"Tree v{__version__}"
    parser.add_argument("DIR", type=path_isdir,
                        help="path of the directory to inspect")
    parser.add_argument("-d", "--dir-only", action="store_true",
                        help="Generate a directory-only tree")
    parser.add_argument(
        "-o",
        "--output-file",
        metavar="OUTPUT_FILE",
        nargs="?",
        help="Generate a full directory tree and save it to a file",
    )
    return parser.parse_args()
