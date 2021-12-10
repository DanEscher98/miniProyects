TARGET = main
FLAGS = -Wall -pedantic -std=c99
HEADERS =

$(TARGET).out: $(TARGET).c
	clang $(TARGET).c $(FLAGS) $(HEADERS) -o target/$(TARGET).out

run: $(TARGET).out
	./target/$(TARGET).out $(USER)	

clear:
	rm -f target/*.out
	clear && ls
