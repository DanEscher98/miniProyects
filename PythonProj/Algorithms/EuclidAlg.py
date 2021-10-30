def Division(m, n):
    def auxDiv(q, a, b):
        if a > b:
            return auxDiv(q+1, a-b, b)
        else:
            return (q, a)
    return auxDiv(0, m, n)


def EuclidGCD(a, b):
    while True:
        q = a // b
        r = a % b
        print("({}, {}, {}, {})".format(a, b, q, r))
        if r == 0: break
        a, b = b, r
    return b
