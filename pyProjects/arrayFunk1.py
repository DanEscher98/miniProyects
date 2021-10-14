import numpy as np
import decimal

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
    	if((num[i]+1)/(int(num[i])+1)==1): 
    		num[i]=int(num[i])
    return num
