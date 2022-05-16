#!/usr/bin/bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"
ALL=$(find $ROOT -type f -name "*.txt")

[[ ! -d "$OUT_DIR" ]] && mkdir --parents "$OUT_DIR"

for file in ${ALL}; do
	BASENAME=$(basename $file .txt)
	head			$file > $OUT_DIR/$BASENAME.txt
	head --lines 5	$file > $OUT_DIR/$BASENAME.l.txt
	head --bytes 30	$file > $OUT_DIR/$BASENAME.c.txt
done

head			$ALL > $OUT_DIR/ALL.txt
head --lines 5	$ALL > $OUT_DIR/ALL.l.txt
head --bytes 30	$ALL > $OUT_DIR/ALL.c.txt
