TARGET = memorybits

$(TARGET).pdf: $(TARGET).md
	pandoc $(TARGET).md --metadata-file=metadata.yaml -o $(TARGET).pdf --standalone

open: $(TARGET).pdf
	evince $(TARGET).pdf

clear:
	rm -rf *.pdf
