#!/usr/bin/bash
#####################################################
#			Setup compiler environment				#
# This is a simple program to perform updates on a	#
# Linux computer. It installs:						#
# - The 'clang' compiler							#
# - The 'make' tool									#
# - Optional: The Haskell interpreter 'ghci'		#
# - Optional: The general markup converter 'pandoc'	#
#####################################################


#######################################
## Select Package Manager #############
function SelectPkgMgr() {
	# get the Distribution, release and architecture.
	if command -v apt &> /dev/null; then
		PkgMgr=apt	# For Debian based distros
		apt update
		apt upgrade
	elif command -v dnf &> /dev/null; then
		PkgMgr=dnf	# For Fedora
		#dnf upgrade
	elif command -v pacman &> /dev/null; then
		PkgMgr=pacman	# For Arch based distros
		pacman -Syu
	else
		Msg="Unable to define the Package Manager."
		error=7
		Quit $error
	fi
	printf "Using \'$PkgMgr\' as Package Manager\n\n"
}


#######################################
## Count how many lines writed ########
function CountLines() {
	for i in $(find . -type f -not -path "./target" \
		| grep -v -E "*.out|*.pdf"); do
		wc -l $i;
	done | sort -n | tee /dev/tty \
		| awk '{ print $2 }' \
		| xargs cat | wc -l \
		| xargs -I % echo "Total lines: " %
}

########################################
## Installation proccess ###############
function InstallProcess() {
	local package=$1
	local executable=$2
	local flags=$3

	echo "Installing $package ..."
	yes | $PkgMgr install $package &> /dev/null
	if command -v $executable; then
		echo "Ok => The package $package has been installed"
	else
		echo "Err => The package $package hasn't been installed"
		ERRORS=$(($ERRORS+1))
	fi
	printf "\n"
}

#######################################
## Handles interactive installation ###
function OptionalInstall() {
	local package=$1
	local executable=$2
	local flags=$3
	echo "Do you want to install $package?"
	read -p "$* [y/n]: " yn
	case $yn in
		[Yy]*) echo "Ok, this may take a little bit ...";
			InstallProcess $package $executable $flags;;
		[Nn]*) printf "Aborted\n\n";;
	esac
}

#######################################
## Main program #######################
GREEN='\033[0;32m'
NC='\033[0m' # No Color
ERRORS=0

# Check for root
if [ `id -u` != 0 ]; then
	echo "┌-------------------------------------------┐"
	echo "| You must be root user to run this program |"
	echo "└-------------------------------------------┘"
	CountLines
	exit 1
fi

SelectPkgMgr
if [ -n "$PkgMgr" ]; then
	InstallProcess python3.9 python3
	InstallProcess clang-13 clang --install-suggests
	InstallProcess make make
	OptionalInstall haskell-platform ghci
	OptionalInstall pandoc pandoc
	OptionalInstall gnuplot gnuplot
else
	exit 1
fi

if [ $ERRORS -eq 0 ]; then
	printf "\nEverything ready!\n"
	echo "Type 'make run' to compile and run the programs"
else
	printf "\nSome packages hasn't been installed.\n"
	echo "=> $ERRORS errors were found"
fi

## End of program #####################
#######################################
