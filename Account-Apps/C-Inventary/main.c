#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

#define MAX 10

struct article {
	char	item[40];
	double	cost;
	double	price;
	int		available;
	int		days;		// number of days until reset
} inventory[MAX];

void init_list(void);
void intro(void);
void print_list(void);
void save(void);
void load(void);
void update(void);
void entry(int);
char menu(void);

int main(void) {
	char option;
	init_list();
	while(1) {
		option = menu();
		switch (option) {
			case 'i' : intro(); break;
			case 'p' : print_list(); break;
			case 'u' : update(); break;
			case 's' : save(); break;
			case 'l' : load(); break;
			case 'q' : exit(0); break;
			default: fputs("Not a valid option.\n", stdout);
		}
	}
	return 0;
}

// Init the array inventory
void init_list(void) {
	for(int i=0; i<MAX; i++) {
		*inventory[i].item = '\0';
	}
}

// Selection menu
char menu(void) {
	char c;
	do {
		fputs("(I)init\n", stdout);
		fputs("(P)rint\n", stdout);
		fputs("(L)load\n", stdout);
		fputs("(U)pdate\n", stdout);
		fputs("(S)ave\n", stdout);
		fputs("(Q)uit\n", stdout);
		c = tolower(getc(stdin));
	} while(!strchr("iplusq", c));
	return c;
}

// Add new elements
void intro(void) {
	int i;
	// Find the first empty structure
	for (i=0; i<MAX; i++) {
		if(!inventory[i].item) break;
	}
	if (i == MAX) {
		fputs("Invetory full\n", stdout);
		return;
	}
	entry(i);
}

// Set the new information
void entry(int i) {
	char aux[80];
	
	printf("item: ");
	fgets(inventory[i].item, 80, stdin);

	printf("cost: ");
	fgets(aux, 80, stdin);
	inventory[i].cost = atof(aux);
	
	printf("available: ");
	fgets(aux, 80, stdin);
	inventory[i].available = atoi(aux);

	printf("days: ");
	fgets(aux, 80, stdin);
	inventory[i].days = atoi(aux);
}

// Modify an item
void update(void) {
	int i;
	char aux[80];

	printf("set item: ");
	fgets(aux, 80, stdin);

	for (i=0; i < MAX; i++) {
		if(!strcmp(aux, inventory[i].item)) break;
	}

	if (i == MAX) printf("item does not exist\n");
	printf("Set new information: ");
	entry(i);
}

// Show list
void print_list(void) {
	for (int i=0; i < MAX; i++) {
		if (inventory[i].item) {
			printf("%s\n", inventory[i].item);
			printf("cost: $%5.2lf - price: $%5.2lf\n",
					inventory[i].cost, inventory[i].price);
			printf("available: %d\n", inventory[i].available);
			printf("days: %d\n\n", inventory[i].days);
		}
	}
}

// Save the list
void save(void) {
	FILE *file;
	if ((file = fopen("inventory.dat", "wb")) == NULL) {
		printf("The file cannot be opened\n");
		return;
	}

	for (int i=0; i<MAX; i++) {
		if (*inventory[i].item) {
			if (fwrite(&inventory[i],
						sizeof(inventory), 1, file) != 1) {
				printf("Error in writing\n");
			}
		}
	}

	fclose(file);
}

// Load file
void load(void) {
	FILE *file;

	if ((file = fopen("inventory.dat", "rb")) == NULL) {
		printf("The file cannot be read");
		return;
	}

	init_list();
	for (int i=0; i < MAX; i++) {
		if (fread(&inventory[i],
					sizeof(inventory), 1, file) != 1) {
			if (feof(file)) {
				fclose(file);
				return;
			}
			printf("Error reading the file.\n");
		}
	}
}
