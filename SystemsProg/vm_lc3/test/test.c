#include "../src/lib.h"
#include <clove.h>

CLOVE_TEST(FirstTest)
{
	int a = 1;
	int b = 1;
	CLOVE_INT_EQ(a, b);
}

CLOVE_RUNNER(FirstTest)
