CC = clang
TARGET  = ttyecho
FLAGS   = -Wall -pedantic -std=c99
HEADERS =

$(TARGET).out: $(TARGET).c
	$(CC) $(TARGET).c $(FLAGS) $(HEADERS) -o $(TARGET).out
	#@sudo chown root:root $(TARGET).out
	#@sudo chmod u+s $(TARGET).out

run: $(TARGET).out
	./$(TARGET).out -d $(shell tty) -c "ls | wc"

test.out: test/tests.c
	$(CC) test/tests.c $(HEADERS) -g -o target/tests.out

test: test.out
	valgrind -s --leak-check=full target/tests.out 2> target/mem_analysis.log
	cat target/mem_analysis.log | cut --complement -d  -f1 | uniq

clear:
	rm -f *.out
	clear && ls
