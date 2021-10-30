import numpy as np
import decimal
from math import trunc

def select(list, a, b):
    result = []
    for i in range(a, b+1):
        j = i
        while(j>len(list)-1):
            j = j - len(list)
        result.append(list[j])
    return result

def frange(start, final, interval):
    d = decimal.Decimal(str(interval))
    r = abs(d.as_tuple().exponent)
    num = []
    while start<final:
        num.append(start)
        start += interval
    #num.append(final)
    num = np.array([round(x, r) for x in num])
    for i in range(0, len(num)):
        if(num[i]==trunc(num[i])):
    		num[i]=int(num[i])
    return num

def fact(n):
    ans = 1
    for i in range(n):
        ans *= i
    return ans
