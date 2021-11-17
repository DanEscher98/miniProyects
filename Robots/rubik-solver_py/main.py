#!/usr/bin/python3
import pycuber
from pycuber.solver.cfop import CrossSolver

mycube = pycuber.Cube()
print(mycube)

# Create a Formula object
my_formula = pycuber.Formula("R U R' U' R' F R2 U' R' U' R U R' F'")
print(my_formula)
print(mycube(my_formula))

# Reversing a Formula
my_formula.reverse()
print(my_formula)
print(mycube(my_formula))

# Mirroring a Formula object
my_formula.mirror("LR")
print(my_formula)
print(mycube(my_formula))

sol = CrossSolver(mycube(my_formula)).solve()
print(sol)
