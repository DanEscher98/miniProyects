# Daniel Sanchez Dominguez
# 9-Oct-2020

from os import system, name
from arrayFunk import frange
import matplotlib.pyplot as plt
import decimal
import numpy as np
import math

def graph(a, b, z, n):
    name = "z:"+str(z)+" n:"+str(n)
    plt.plot(a, b, 'ro')
    plt.xlabel('Re axis')
    plt.ylabel('Im axis')
    plt.title("The {} roots of {}".format(n, z))
    plt.axis('scaled')
    plt.grid(True)
    plt.rc('grid', linestyle="-", color='black')
    handles, labels = ax.get_legend_handles_labels()
    lgd = ax.legend(handles, labels, loc='upper left', bbox_to_anchor=(1, 1))
    plt.savefig('Output/'+name+'.png', bbox_extra_artists=(lgd,), bbox_inches='tight')
    plt.show()

def roothe(r, a, n):
    path_a, path_b , pA, pB = [], [], [], []
    for k in frange(1, n, 0.01):
        path_a.append(pow(r, k)*math.cos(k*a))
        path_b.append(pow(r, k)*math.sin(k*a))
        if math.modf(k)[0] == 0:
            pA.append(path_a[-1])
            pB.append(path_b[-1])
    return (path_a, path_b, pA, pB)

def calc_nthr(z, n):
    r = math.sqrt(z.real**2 + z.imag**2)
    theta = math.atan2(z.imag, z.real)
    a, b = [], []
    for k in range(n):
        arg = (theta + 2*np.pi*k)/n
        absv = pow(r, 1.0/n)
        a.append(absv*math.cos(arg))
        b.append(absv*math.sin(arg))
        path_a, path_b, pA, pB = roothe(absv, arg, n)
        ax.plot(path_a, path_b, label="{:.4f} + {:.4f}j\n".format(a[k], b[k]))
        ax.plot(pA, pB, 'ko')
    plt.plot(z.real, z.imag, 'bo', label="{}".format(str(z)))
    return (a, b)


# Codigo para interfaz del usuario
def clear():
    if name == 'nt': system('cls')
    else: system('clear')

def menu():
    try:
        z = complex(input("Set a complex number: "))
        n = int(input("Set value of n: "))
    except ValueError:
        print('Has ingresado valores erroneos')
    else:
        global f, ax
        f, ax = plt.subplots(1)
        a, b = calc_nthr(z, n)
        graph(a, b, z, n)

if __name__=='__main__':
    while True:
        print('Daniel Sanchez Dominguez - 1707549\n')
        menu()
        if input('¿Terminar programa? (y/n): ')=='y':
            break
        clear()
