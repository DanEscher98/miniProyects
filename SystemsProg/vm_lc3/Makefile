TARGET = main
FLAGS	= -Wall -pedantic -std=c99
TEST_F	= $(FLAGS) -lcheck -pthread -lcheck_pic -lrt -lm -lsubunit
HEADERS =
CC		= clang

$(TARGET).out: src/$(TARGET).c
	$(CC) src/$(TARGET).c $(FLAGS) $(HEADERS) -o target/$(TARGET).out

run: $(TARGET).out
	./target/$(TARGET).out $(USER)

target/test.out: test/test.c
	$(CC) test/test.c $(TEST_F) -o target/test.out

test: target/test.out
	./target/test.out

clear:
	rm -f target/*.out
	clear && ls
