#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 40

struct direction {
	char name[30];
	char street[40];
	char city[20];
	char country[5];
	char postal_code[10];
} list[MAX];

void init_list(void) {
	for (register int i=0; i < MAX; i++) {
		*list[i].name = '\0';
	}
}

int menu(void) {
	char s[10];
	int c;
	
	fputs("1. Set a new entry\n", stdout);
	fputs("2. Delete an entry\n", stdout);
	fputs("3. Show list\n", stdout);
	fputs("4. Search a direction\n", stdout);
	fputs("5. Save list in a file\n", stdout);
	fputs("6. Read list from file\n", stdout);

	do {
		printf("Choose an option\n", stdout);
		fgets(s, 10, stdin);
		c = atoi(s);
	} while (c < 0 || c > 7);
	return c;
}



void new_entry(void) {
	int i;
	while(1) {
		i = find_free();
		if (i < 0) {
			printf("No empty space.\n");
			return;
		}

		get_answer("Set name: ",		list[i].name);
		get_answer("Set street: ",		list[i].street);
		get_answer("Set city: ",		list[i].city);
		get_answer("Set country: ",		list[i].country);
		get_answer("Set postal code: ", list[i].postal_code);
	}
}


int main(void) {
	int option;
	init_list();

	for(;;) {
		option = menu();
		switch(option) {
			case 1: new_entry();	break;
			case 2: delete_entry(); break;
			case 3: show_menu();	break;
			case 4: search_entry(); break;
			case 5: save_list();	break;
			case 6: read_list();	break;
			case 7: exit(0);
			default: fputs("Not a valid option\n", stdin); break;
		}
	}
}
