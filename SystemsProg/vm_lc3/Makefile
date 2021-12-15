TARGET = main
FLAGS	= -Wall -pedantic -std=c99
HEADERS =
CC		= clang

$(TARGET).out: src/$(TARGET).c
	$(CC) src/$(TARGET).c $(FLAGS) $(HEADERS) -o target/$(TARGET).out

run: $(TARGET).out
	./target/$(TARGET).out $(USER)

target/tests.out: test/test.c
	$(CC) test/test.c -g -o target/tests.out

test: target/tests.out
	valgrind -s --leak-check=full target/tests.out 2> target/mem_analysis.log
	cat target/mem_analysis.log

clear:
	rm -f target/*.out target/*.log
	clear && ls
