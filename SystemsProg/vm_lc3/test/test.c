#include "../src/lib.h"
#include <check.h>

START_TEST(basic) { ck_assert_int_eq(1, 1); }
END_TEST

int main(void)
{
	Suite* s1 = suite_create("Core");
	TCase* tc1_1 = tcase_create("Core");
	SRunner* sr = srunner_create(s1);
	int nf;

	suite_add_tcase(s1, tc1_1);
	tcase_add_test(tc1_1, basic);
	// tcase_add_test(tc1_1, other);
	srunner_run_all(sr, CK_ENV);
	nf = srunner_ntests_failed(sr);
	srunner_free(sr);

	return nf == 0 ? 0 : 1;
}
