# Danyiel Colin 11/Nov/20

from os import system
import math

global acc
acc = 0.0001


def integration(a, b):
    n = int((b - a) / acc)
    delta_x = (b - a) / n
    s = 0
    for i in range(1, n):
        x_i = a + (i - 0.5) * delta_x
        s += f(x_i) * delta_x
    return s


def f(x):
    return math.sin(x)


if __name__ == "__main__":
    while True:
        a = float(input("Set a: "))
        b = float(input("Set b: "))
        ans = integration(a, b)
        print("I[f(x)] with\na = {}\nb = {}".format(a, b))
        print("is equal to: {}".format(ans))
        if input("Finish program?: ") == "y":
            break
        clear()
