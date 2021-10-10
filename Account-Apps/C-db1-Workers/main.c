#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>
#define DATA "account.dat"

void intro_account(void);
void find(void);
char menu(void);

int main(void) {
	char option;
	do {
		option = menu();
		switch(option) {
			case 'i':
				intro_account();
				break;
			case 'b':
				find();
				break;
		}
	} while (option != 'q');
	return 0;
}

char menu(void) {
	char car;
	do {
		fputs("(I)ntroduce\n(B)rowse\n(Q)uit\n", stdout);
		fputs("Select an option: ", stdout);
		car = tolower(getc(stdin));
		printf("\n");
	} while(car != 'i' && car != 'b' && car != 'q');
	return car;
}

void intro_account(void) {
	FILE *file;
	char name[80];
	double salary;

	/* Open the file to add an element */
	if  ((file = fopen(DATA, "a")) == NULL) {
		fputs("File can't be opened\n", stdout);
		exit(1);
	} else {
		puts("Set the name and salary: ");
		fscanf(stdin, "%s%lf", name, &salary);
		fscanf(stdin, "%*c");

		fprintf(file, "%s %lf\n", name, salary);
		fclose(file);
	}
}

void find(void) {
	FILE *file;
	char name[80], aux_name[80];
	double salary;

	// Open file to read
	if ((file = fopen(DATA, "r")) == NULL) {
		fputs("The file could not be opened", stdout);
		exit(1);
	}

	fputs("Set name to find: ", stdout);
	scanf("%s", name);
	// Find account and salary
	while (!feof(file)) {
		fscanf(file, "%s%lf", aux_name, &salary);
		if (!strcmp(name, aux_name)) {
			printf("%s: $%7.2lf\n", name, salary);
			break;
		}
	}
	printf("\n");
	fclose(file);
}
