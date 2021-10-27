TARGET 	= text_processor
FLAGS	= -Wall -Wextra -pedantic

$(TARGET).out : $(TARGET).c
	clang $(TARGET).c -o $(TARGET).out $(FLAGS)

run: $(TARGET).out
	./$(TARGET).out edit test.txt

clear :
	rm *.out

