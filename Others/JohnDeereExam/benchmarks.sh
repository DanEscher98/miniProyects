#!/usr/bin/bash

PROGRAM=spayer_path

rm -f target/$PROGRAM.dat

for i in $(seq 1000000); do
	start_t=$(date +%s%N)

	./target/$PROGRAM.out $i &> /dev/null
	
	final_t=$(date +%s%N)
	diff_t=$((($final_t - $start_t)/1000))
	echo "$i a:$start_t b:$final_t d:$diff_t" \
		>> target/$PROGRAM.dat
done
