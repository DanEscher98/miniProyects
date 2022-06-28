# Danyiel Colin 11/Nov/20

import math
from typing import Callable

global acc
acc = 0.0001


def integration(fn: Callable[[float], float], a, b) -> float:
    n = int((b - a) / acc)
    delta_x = (b - a) / n
    s = 0
    for i in range(1, n):
        x_i = a + (i - 0.5) * delta_x
        s += fn(x_i) * delta_x
    return s


if __name__ == "__main__":
    while True:
        a = float(input("Set a: "))
        b = float(input("Set b: "))
        ans = integration(math.sin, a, b)
        print(f"I[f(x)]({a}, {b}) = {ans:.4f}")
        if input("Finish program?: ") == "y":
            break
