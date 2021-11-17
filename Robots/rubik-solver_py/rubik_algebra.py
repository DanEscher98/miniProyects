#!/usr/bin/python3
import numpy as np


def init_matrix(value, m, n):
    return [[value]*m]*n


mycube = np.array(
    [init_matrix(0, 3, 3),
     init_matrix(1, 3, 3),
     init_matrix(2, 3, 3),
     init_matrix(3, 3, 3),
     init_matrix(4, 3, 3),
     init_matrix(5, 3, 3)])

print(mycube)


def fullturn_x(cube):
    print(cube)
