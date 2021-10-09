TARGET 	= MINI_PROJECT_IN_C
FLAGS	= -Wall -Wextra -pedantic

$(TARGET).out : $(TARGET).c
	clang -o $(TARGET).out $(TARGET).c $(FLAGS)

run : $(TARGET).out
	./$(TARGET).out

clean :
	rm *.out
