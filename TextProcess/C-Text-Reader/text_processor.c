#include <stdio.h>
#include <stdlib.h>

char *validate_name(char *name) {
	return name;
}

FILE *get_file(char* mode, char* name) {
	FILE *pf;
	if((pf = fopen(name, "w+")) == NULL) {
		fputs("File can't be", stdout);
		exit(1);
	}
	return pf;
}

void edit_file(FILE *file) {
	char car;
	do {
		car = getc(stdin);
		putc(car, file);
	} while (car != '%');
}

void print_file(FILE *file) {
	char car;
	while(!feof(file)) {
		car = getc(file);
		putc(car, stdout);
	}
}

int main(int argc, char *argv[]) {
	FILE *file;
	if(argc != 3) {
		fputs("Need a mode and a file name\n", stdout);
		exit(1);
	}
	// Check if file exists
	file = get_file(argv[1], argv[2]);
	print_file(file);
	edit_file(file);
	fclose(file);
	return 0;
}
