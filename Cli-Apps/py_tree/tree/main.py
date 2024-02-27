from tree.core import DirectoryTree
from tree.cli import parse_args
import sys


def main():
    args = parse_args()
    tr = DirectoryTree(
            args.DIR,
            dir_only=args.dir_only,
            output_file=args.output_file if args.output_file else sys.stdout
            )
    tr.generate()
