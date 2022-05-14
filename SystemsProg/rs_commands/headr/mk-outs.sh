#!/usr/bin/bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"
ALL=$(find $ROOT -type f -name "*.txt")

[[ ! -d "$OUT_DIR" ]] && mkdir --parents "$OUT_DIR"

for file in ${ALL}; do
	BASENAME=$(basename $file .txt)
	cat		$file > $OUT_DIR/$BASENAME.txt
	cat -b	$file > $OUT_DIR/$BASENAME.b.txt
	cat -n	$file > $OUT_DIR/$BASENAME.n.txt
done

cat		$ALL > $OUT_DIR/ALL.txt
cat -b	$ALL > $OUT_DIR/ALL.b.txt
cat -n	$ALL > $OUT_DIR/ALL.n.txt
