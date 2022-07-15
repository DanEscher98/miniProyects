"""Provides the cli interface"""

import argparse
import math
from argparse import Namespace
from typing import Dict, Tuple

from cntlang import Colors


def args_namespace() -> Namespace:
    """
    - path: StrPath
    - Sort by: lines, files, size (default lines)
    - humand readable: bool (default True)
    - inverse order:   bool (default False)
    - only since date: date (default since ever)
    - only trash file: bool (default False)
    """
    parser = argparse.ArgumentParser(
        prog="cntlang",
        usage="%(prog)s [options] path",
        description="Count lines from source code files",
        epilog="Keep the good coding effort!",
    )
    parser.add_argument("Path", metavar="path", type=str, help="the path to list")
    parser.add_argument(
        "-b", "--bytes", action="store_true", help="show size in plain bytes"
    )
    parser.add_argument(
        "-s",
        "--sort-by",
        type=str,
        default="line",
        choices=["line", "file", "size"],
        help="sort by lines, files or size",
    )
    parser.add_argument(
        "-t", "--since-date", help="only show files modified since date"
    )
    parser.add_argument(
        "-i",
        "--inverse-order",
        action="store_false",
        help="set if normal or inverse order",
    )
    return parser.parse_args()


def human_readable(byte_sz: int) -> str:
    kb, mb, gb = 1024, math.pow(1024, 2), math.pow(1024, 3)
    if byte_sz > gb:
        size_hr = f"{(byte_sz / gb):.2f}G"
    elif byte_sz > mb:
        size_hr = f"{(byte_sz / mb):.2f}M"
    elif byte_sz > kb:
        size_hr = f"{(byte_sz / kb):.2f}K"
    else:
        size_hr = f"{byte_sz}B"
    return size_hr


def format_output(file_data: Dict[str, Tuple], args):
    columns = {"line": 0, "file": 1, "size": 2}
    by_value = lambda item: item[1][columns[args.sort_by]]
    colored = lambda string, color: f"{color}{string}{Colors.ENDC}"
    humanread = lambda num: num if args.bytes else human_readable(num)
    total_l, total_f, total_s = 0, 0, 0

    lang, lines, files, size = map(
        lambda st: colored(st, Colors.GREEN), ["LANGUAGE", "LINES", "FILES", "SIZE"]
    )
    print(f"{lang:<15} {lines:>18} {files:>16} {size:>18}")
    for lang, (lines, files, size) in sorted(
        file_data.items(), key=by_value, reverse=args.inverse_order
    ):
        total_l += lines
        total_f += files
        total_s += size
        print(f"{lang:<10} {lines:8} {files:8} {humanread(size):>10}")
    total = colored("Total", Colors.RED)
    print(f"{total:<18} {total_l:8} {total_f:8} {humanread(total_s):>10}")
