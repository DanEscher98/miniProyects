import os

from cntlang.app import classify_files
from cntlang.cli import args_namespace, format_output


def main():
    args = args_namespace()
    file_data = classify_files(os.getcwd())
    format_output(file_data, args)


if __name__ == "__main__":
    main()
