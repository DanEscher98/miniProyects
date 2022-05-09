#!/usr/bin/python3

import math
import sys


def to_pint(n):
    n = abs(n)

    def to_infinity():
        index = 0
        while True:
            yield index
            index += 1

    for i in to_infinity():
        ans = float(n * 10 ** i)
        if ans.is_integer():
            return int(ans)


def one_digit_p(n):
    n = float(abs(n))
    if (n / 10 < 1) and (n.is_integer()):
        return True
    else:
        return False


def list_digits(n):
    n = to_pint(n)
    ans = []
    while n != 0:
        ans.append(n % 10)
        n = int(n / 10)
    if ans[0] == 0:
        del ans[0]
    return ans


def sum_digits(n):
    def loop(d_list):
        ans = sum(d_list)
        if one_digit_d(ans):
            return ans
        else:
            d_list = list_digits(ans)
            return loop(d_list)

    d_list = list_digits(n)
    print(d_list)
    ans = loop(d_list)
    return ans


def maping(func, lst):
    ans = []
    for elem in lst:
        ans.append(func(elem))
    return ans


if __name__ == "__main__":
    arg = maping(float, sys.argv[1:])
    print(sum_digits(arg[0]))
