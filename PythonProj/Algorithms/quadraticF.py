from os import system, name


def clear():
    if name == "nt":
        system("cls")
    else:
        system("clear")


def roots():
    a = float(input("Enter a: "))
    b = float(input("Enter b: "))
    c = float(input("Enter c: "))
    D = b * b - 4 * a * c
    if D == 0:
        x = (-b) / (2 * a)
        print("x: {0}".format(x))
        return
    elif D > 0:
        D = D ** 0.5
    else:
        D = ((D * -1) ** 0.5) * 1j
    x_1 = (-b + D) / (2 * a)
    x_2 = (-b - D) / (2 * a)
    print("x1: {0:.4f}".format(x_1))
    print("x2: {0:.4f}".format(x_2))


if __name__ == "__main__":
    while True:
        clear()
        roots()
        if input("Finish? (y/n): ") == "y":
            break
