/* author: Daniel Sánchez Domínguez
 * date: 27-Oct-2021*/
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include "lists_vectors.h"

int main(int argc, char* argv[argc+1]) {
	srand(time(NULL));
	int size_a, size_b;
	bool sorted = false;
	if (argc > 2) {
		size_a = parseInt(argv[1]);
		size_b = parseInt(argv[2]);
	} else {
		size_a = 10+rand()%90;
		size_b = 10+rand()%90;
	}
	if (argc == 4) {
		if (!strcmp(argv[3], "s")) sorted = true;
	}
	// Initializing vector and list with random values
	vector vec_a = initVector(size_a, false, sorted);
	list ls_b = initRandomList(size_b, sorted);
	
	printf("TRACTORS VIN NUMBER\n\n");
	printf("Vector A (size: %d):\n",
			vec_a.length);
	printVector(vec_a);
	putc('\n', stdout);
	printf("List B (size: %d):\n",
			ls_b.length);
	printList(ls_b);
	putc('\n', stdout);
	list ls_c;
	ls_c = intersection(vec_a, ls_b);
	printf("Intersection of both lists (size: %d):\n",
			ls_c.length);
	printList(ls_c);

	free(vec_a.values);
	freeList(ls_c);
	horizontalLine();
	return EXIT_SUCCESS;
}
