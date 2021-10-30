#!/usr/bin/python3
"""
This program could be executed through the commmand line
or as a cli-app.

Usage:
search_header.py --path PATH --name NAME

If no arguments are given, the program will request them to
the user.
"""

import os, re, argparse

#######################################
## Define parser for flag args ########
parser = argparse.ArgumentParser()
parser.add_argument("--path",
                    help="Set the search path",
                    action="store")
parser.add_argument("--name",
                    help="Search by a given name",
                    action="store")


#######################################
## Filter files with a regex ##########
def filter(name, files):
    for file in files:
        if re.search(f'{name}*\.[ch]$', file):
            yield file


#######################################
## Get files recursively ##############
def get_files(path):
    subfolders, files = [], []

    for f in os.scandir(path):
        if f.is_dir():
            subfolders.append(f.path)
        if f.is_file():
            files.append(f.path)

    for sub_dir in list(subfolders):
        sf, f = get_files(sub_dir)
        subfolders.extend(sf)
        files.extend(f)
    return subfolders, files


#######################################
## Main Program #######################
if __name__=='__main__':
    args = parser.parse_args()

    if args.name and args.path:
        name = args.name
        path = args.path
    else:
        path = input("Set the search path: ")
        name = input("Set the name to search: ")
    try:
        _, files = get_files(path)
        print('\n'.join(list(filter(name, files))))
    except:
        print("The path does not exist")
