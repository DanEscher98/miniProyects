TARGET = problems
EXT = md

pdf: $(TARGET).$(EXT)
	pandoc $(TARGET).$(EXT) -o $(TARGET).pdf --standalone --toc
	cp $(TARGET).pdf $(HOME)/MyWindows/Working/

clear:
	rm -f *.pdf
	clear && ls
