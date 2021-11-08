FILE_1	= tractors_number
FILE_2	= spayer_path
FILE_3	= search_c
SPATH 	= source
TPATH 	= target

SPATH_1	= $(SPATH)/$(FILE_1)
TPATH_1 = $(TPATH)/$(FILE_1)
SPATH_2	= $(SPATH)/$(FILE_2)
TPATH_2 = $(TPATH)/$(FILE_2)
SPATH_3 = $(SPATH)/$(FILE_3)

HEADERS	= $(SPATH)/lists_vectors.c
FLAGS 	= -Wall -pedantic -std=c99

$(TPATH_1).out : $(SPATH_1).c
	clang $(SPATH_1).c $(FLAGS) $(HEADERS) -o $(TPATH_1).out

$(TPATH_2).out : $(SPATH_2).c
	clang $(SPATH_2).c $(FLAGS) $(HEADERS) -o $(TPATH_2).out

$(SPATH_3).py :
	chmod +x $(SPATH_3).py
	chmod +x alternative/search_c.sh
	chmod +x benchmarks.sh

run: $(TPATH_1).out $(TPATH_2).out $(SPATH_3).py
	./$(TPATH_1).out
	./$(TPATH_2).out
	./$(SPATH_3).py --path "/usr/include/" --name "stdio"

html: README.md
	pandoc README.md -o README.html --standalone --toc --mathjax

docx: README.md
	pandoc README.md -o README.docx --toc

pdf: README.md
	pandoc README.md -o README.pdf --standalone --toc

clear:
	rm -f ./target/*.out *.pdf *.html *.docx
	clear && ls
