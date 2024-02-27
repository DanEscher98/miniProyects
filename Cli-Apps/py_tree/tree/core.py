import os
import sys
import pathlib
from pathlib import Path
from enum import Enum


class DrawAscii (Enum):
    BAR = "─"
    ELBOW = "└──"
    PIPE = "|"
    TEE = "├──"
    PIPE_PREFIX = "|   "
    SPACE_PREFIX = "    "


class DirectoryTree:
    def __init__(self, root_dir, output_file, dir_only=False):
        self._output_file = output_file
        self._generator = _TreeGenerator(root_dir, dir_only)

    def generate(self):
        tree = self._generator.build_tree()
        if self._output_file != sys.stdout:
            tree.insert(0, "```")
            tree.append("```")
            with open(self._output_file, mode="w", encoding="UTF-8") as stream:
                for entry in tree:
                    print(entry, file=stream)
        else:
            for entry in tree:
                print(entry, file=self._output_file)


class _TreeGenerator:
    def __init__(self, root_dir, dir_only: bool):
        self._root_dir = pathlib.Path(root_dir)
        self._dir_only = dir_only
        self._tree = []

    def build_tree(self):
        self._tree_head()
        self._tree_body(self._root_dir)
        return self._tree

    def _tree_head(self):
        self._tree.append(f"{self._root_dir}{os.sep}")
        self._tree.append(DrawAscii.PIPE.value)

    def _tree_body(self, directory: Path, prefix=""):
        entries = directory.iterdir()
        entries = sorted(entries, key=lambda entry: entry.is_file())
        entries = [entry for entry in entries if entry.is_dir()] \
            if self._dir_only else entries
        entries_count = len(entries)

        for index, entry in enumerate(entries):
            connector = DrawAscii.ELBOW.value if index == entries_count - 1 \
                    else DrawAscii.TEE.value
            if entry.is_dir():
                self._add_directory(
                        entry, index, entries_count, prefix, connector)
            elif not self._dir_only:
                self._add_file(entry, prefix, connector)

    def _add_directory(
            self, directory: Path, index, entries_count, prefix, connector
            ):
        self._tree.append(f"{prefix}{connector} {directory.name}{os.sep}")
        if index != entries_count - 1:
            prefix += DrawAscii.PIPE_PREFIX.value
        else:
            prefix += DrawAscii.SPACE_PREFIX.value
        self._tree_body(directory=directory, prefix=prefix)
        self._tree.append(prefix.rstrip())

    def _add_file(self, file: Path, prefix, connector):
        self._tree.append(f"{prefix}{connector} {file.name}")
