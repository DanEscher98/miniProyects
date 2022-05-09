from numpy import zeros


def pascal_triangle(n):
    P = zeros((n, n), dtype=int)
    for i in range(n):
        for j in range(i + 1):
            if j == 0 or j == i:
                elem = 1
            else:
                elem = P[i - 1][j - 1] + P[i - 1][j]
            P[i][j] = elem
    return P
