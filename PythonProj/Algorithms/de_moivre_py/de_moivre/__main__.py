# Daniel Sanchez Dominguez
# 9-Oct-2020

from os import system, name
from de_moivre.discrete_array import frange
import matplotlib.pyplot as plt
import numpy as np
import math
from typing import Tuple, List, Iterable
from dataclasses import dataclass


def graph(a, b, z, n):
    name = "z:" + str(z) + " n:" + str(n)
    plt.plot(a, b, "ro")
    plt.xlabel("Re axis")
    plt.ylabel("Im axis")
    plt.title("The {} roots of {}".format(n, z))
    plt.axis("scaled")
    plt.grid(True)
    plt.rc("grid", linestyle="-", color="black")
    handles, labels = ax.get_legend_handles_labels()
    lgd = ax.legend(handles, labels, loc="upper left", bbox_to_anchor=(1, 1))
    plt.savefig(
        "Output/" + name + ".png",
        bbox_extra_artists=(lgd,),
        bbox_inches="tight"
    )
    plt.show()


@dataclass
class MoivrePath:
    paths: Iterable[Tuple[float, float]]
    points: Iterable[Tuple[float, float]]


@dataclass
class DeMoivreRoots:
    number: complex
    power: int
    paths: Iterable[Tuple[complex, MoivrePath]]


def roothe(r, a, n) -> MoivrePath:
    path_a, path_b, points_a, points_b = [], [], [], []
    for k in frange(1, n, 0.01):
        path_a.append(pow(r, k) * math.cos(k * a))
        path_b.append(pow(r, k) * math.sin(k * a))
        if math.modf(k)[0] == 0:
            points_a.append(path_a[-1])
            points_b.append(path_b[-1])
    return MoivrePath(zip(path_a, path_b), zip(points_a, points_b))


def calc_nthr(z: complex, n: int) -> Iterable[Tuple[complex, MoivrePath]]:
    r = math.sqrt(z.real ** 2 + z.imag ** 2)
    theta = math.atan2(z.imag, z.real)
    for k in range(n):
        arg: float = (theta + 2 * np.pi * k) / n
        absv: float = pow(r, 1.0 / n)

        root = absv * (math.cos(arg) + math.sin(arg)*1j)
        path = roothe(absv, arg, n)

        yield root, path

        # ax.plot(path_a, path_b, label=f"{root}\n")
        # ax.plot(points_a, points_b, "ko")

    # plot the discrete points
    # plt.plot(z.real, z.imag, "bo", label="{}".format(str(z)))
    # return DeMoivreRoots(z, n, roots, paths)


# Codigo para interfaz del usuario
def clear():
    if name == "nt":
        system("cls")
    else:
        system("clear")


def menu():
    try:
        z = complex(input("Set a complex number: "))
        n = int(input("Set value of n: "))
    except ValueError:
        print("Has ingresado valores erroneos")
    else:
        global f, ax
        f, ax = plt.subplots(1)
        a, b = calc_nthr(z, n)
        graph(a, b, z, n)


if __name__ == "__main__":
    while True:
        menu()
        if input("¿Terminar programa? (y/n): ") == "y":
            break
        clear()
