#Daniel Sanchez Dominguez

import matplotlib.pyplot as plt
import numpy as np
from fractions import Fraction
from os import system, name
import math

# Codigo relevante para los calculos
def draw_graph(x, y, name):
    plt.plot(x, y,label=name)
    plt.xlabel('tiempo')
    plt.ylabel('desplazamiento')
    plt.title('Movimiento de particula en M.A.S.')
    plt.legend(loc="upper right")
    plt.savefig('Output/'+name+'.png')
    plt.show()


def frange(start, final, interval):
    num = []
    while start<=final:
        num.append(start)
        start += interval
    numbers = np.array([round(x, 2) for x in num])
    return numbers

def calc_tab(A, m, k, b, q):
    name = 'A = '+str("{:.2f}".format(A))+\
    '\nw = '+str("{:.2f}".format(math.sqrt(k/m)))
    time = frange(0, 10, 0.01); x=[]
    for t in time: x.append(ec_MAS(t, A, m, k, b, q))
    t = np.array(t); print(t)
    x = np.array(x); print(x)
    draw_graph(time, x, name)

def ec_MAS(t, A, m, k, b, q):
    tm = t/(2*m)
    eb = math.exp(b*tm)
    y  = (A/eb)*math.cos(tm*math.sqrt(4*m*k-b**2))
    return y

# Codigo para interfaz del usuario
def clear():
    if name == 'nt': system('cls')
    else: system('clear')
df
def menu():
    try:
        A = float(Fraction(input('Introduce amplitud: ')))
        m = float(Fraction(input('Introduce masa: ')))
        k = float(Fraction(input('Introduce constante elástica: ')))
        b = float(Fraction(input('Introduce constante fricción: ')))
        q = float(Fraction(input('Introduce angulo de fase: ')))
    except ValueError:
        print('Has ingresado valores erroneos')
    else:
        calc_tab(A, m, k, b, q)

if __name__=='__main__':
    while True:
        print('Daniel Sanchez Dominguez - 1707540\n')
        menu()
        if input('¿Terminar programa? (y/n): ')=='y':
            break
        clear()
