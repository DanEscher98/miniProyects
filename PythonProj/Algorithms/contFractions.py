import numpy as np
from sympy import *
from arrayFunk import *
import matplotlib.pyplot as plt

class contFraction:
    def __init__(self, a, b):
        self.list = self._get(a, b)
        self.aprox = self._eval(self.list)

    def _get(self, a, b):
        s = np.prod(np.sign([a, b]))
        a, b = abs(a), abs(b)
        result = []
        while(b > 0):
            q = 0
            while(not(a<b*(q+1))): q+=1
            result.append(q*s)
            aux = a; a = b
            b = aux-q*b
        return result

    def _eval(self, list):
        result = []
        for i in range(0, len(list)):
            aux = self.calcS(select(list, 0, i))
            result.append(aux)
        return result

    def calcS(self, list):
        x = symbols('x')
        ans = list[0]+1/x
        for i in range(1, len(list)):
            ans = ans.subs(x, list[i]+1/x)
        return ans.subs(1/x, 0)

    def calcN(list):
        ans = 0
        for i in range(1, len(list)):
            ans = 1/(list[-1*i]+ans)
        return list[0]+ans

#==================================================================

def LagrangePol(values):
    x = symbols('x')
    suma = 0; n = len(values)
    for k in range(0, n):
        prod = 1
        for i in range(0, n):
            if not(i==k):
                num = x-values[i][0]
                den = values[k][0]-values[i][0]
                prod *= num/den
        suma += values[k][1]*prod
        suma = simplify(suma)
    return suma

#==================================================================

class asocPol:
    def __init__(self, a, b):
        if abs(a)>abs(b): a, b = b, a
        x = symbols('x')
        self.p = self._p(a, b)
        self.q = self._p(b, a)
        self.pq = self._pq(self.p, self.q)
        self.deg = 2*degree(self.q, gen=x)+1
    def _p(self, a, b):
        ans = contFraction(a, b)
        val = []
        for k in range(0, len(ans.list)):
            val.append((k, ans.aprox[k]))
    def _pq(self, p, q):
        x = symbols('x')
        pq = q*p.subs(x, x+1)
        return simplify(pq)

#==================================================================

def draw(x, p, q, pq, a, b, s):
    name = "(a:"+str(s*a)+", b:"+str(b)+")"
    aproxp = contFraction(a, b).aprox
    aproxq = contFraction(b, a).aprox
    pointP, pointQ, pointPQ , xp = [], [], [], []
    for i in range(0, len(x)):
        if ((x[i]+1)/(int(x[i])+1)==1):
            pointP.append(float(p[i]))
            pointQ.append(float(q[i]))
            pointPQ.append(float(pq[i]))
            plt.annotate(str(s*aproxp[int(x[i])]), (x[i], p[i]))
            if x[i]<len(aproxq):
                num = round(q[i], 4)
                if num/int(num)==1: num=int(num)
                plt.annotate(str(s*num), (x[i], q[i]))
            xp.append(x[i])
    plt.plot(x, p, x, q, x, pq, xp, pointP,'ko', xp, pointQ,'ko', xp, pointPQ,'ko')
    plt.xlabel('x axis')
    plt.ylabel('Aproximates')
    plt.title("The associated polynomials of ({}, {})".format(a, b))
#    plt.yscale("log")
    plt.grid(True)
    plt.rc('grid', linestyle="-", color='black')
    plt.savefig('Output/'+name+'.png')
    plt.show()

#==================================================================
if __name__ == '__main__':
    pair = input("Set a pair of integers: ")
    pair = pair.split(" ")
    a = int(pair[0])
    b = int(pair[1])
    ab = asocPol(a, b)
    s = sign(a/b)
    a,b = abs(a),abs(b)
    if a>b: a,b = b,a
    print("\n{}/{} ~ p(x) = \n{}".format(s*a, b, ab.p))
    print("\n{}/{} ~ q(x) = \n{}".format(s*b, a, ab.q))
    print("\nq(n)*p(n+1) = \n{}\n".format(ab.pq))

    x = symbols('x')
    cardL = (ab.deg+1)/2
    xaxis = frange(0, cardL, 0.01)
    p, q, pq = [], [], []
    for i in xaxis:
        p.append(ab.p.subs(x, i))
        q.append(ab.q.subs(x, i))
        pq.append(ab.pq.subs(x, i))

    draw(xaxis, p, q, pq, a, b, s)
