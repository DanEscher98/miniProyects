from decimal import Decimal
from math import trunc
from typing import Iterable


def select(list, a: int, b: int) -> Iterable:
    for i in range(a, b + 1):
        j = i
        while j > len(list) - 1:
            j = j - len(list)
        yield list[j]


def frange(start, final, interval) -> Iterable[int | float]:
    match Decimal(str(interval)).as_tuple().exponent:
        case int(e):
            r = abs(e)
        case _:
            raise Exception

    while start < final:
        ret: float = round(start, r)
        if ret == trunc(start):
            yield int(ret)
        yield start
        start += interval
    # num.append(final)
