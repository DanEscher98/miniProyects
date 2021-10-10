TARGET 	= main
FLAGS	= -Wall -Wextra -pedantic

$(TARGET).out : $(TARGET).c
	clang $(TARGET).c -o $(TARGET).out $(FLAGS)

run: $(TARGET).out
	./$(TARGET).out

clear :
	rm *.out

