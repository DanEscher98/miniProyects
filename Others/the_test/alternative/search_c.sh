#!/usr/bin/bash

#######################################
## HELP ###############################
function Help() {
	# Display Help
	printf "searchFile - Finds C files and headers
	by a given extension and path to find."
	printf "Syntax: doUpdates --[h|p|n]\n"
	printf "Options:
	h Print this Help.
	p Set the search path
	n Set the name of file"
	printf '\n'
}

#######################################
## Main Program #######################

while getopts 'p:n:h' flag; do
	case "${flag}" in
		h) Help;;
		p) path=${OPTARG:-"/usr/include/"};;
		n) name=${OPTARG:-"stdio"};;
		*) Help; exit 1;;
	esac
done

find $path -type f -name "*$name*\.[ch]"
