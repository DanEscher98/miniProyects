/* author: Daniel Sánchez Domínguez
 * date: 28-Oct-2021*/
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include "lists_vectors.h"

int main(int argc, char *argv[]) {
	srand(time(NULL));
	int size;
	bool sorted = false;
	if (argc > 1) {
		size = parseInt(argv[1]);
	} else {
		// After 15000 as maxium list size, the program
		// starts taking more than 1 second to run
		size = 10+rand()%90;
	}
	if (argc == 3) {
		if (!strcmp(argv[2], "s")) sorted = true;
	}
	
	// Initializing list with random values
	list ls_a = initRandomList(size, sorted);
	
	printf("SPRAYER PATH PLANNING\n\n");
	printf("Original list (size: %d):\n",
			ls_a.length);
	printList(ls_a);

	ls_a = deleteClones(ls_a);
	putc('\n', stdout);
	printf("All clones have been removed (size: %d):\n",
			ls_a.length);
	printList(ls_a);
	freeList(ls_a);
	horizontalLine();
	return 0;
}
