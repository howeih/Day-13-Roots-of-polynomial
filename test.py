import numpy as np
def roots(*coeffs):
    matrix = np.eye(len(coeffs) - 1, k=-1)
    # print(matrix, type(matrix))
    # print(coeffs[:0:-1], type(coeffs))
    # print(coeffs[:0:-1])
    # print( np.array(coeffs[:0:-1]))
    # print(-coeffs[0])
    # print(coeffs)
    # print("1::",matrix)
    matrix[:,-1] = np.array(coeffs[:0:-1]) / -coeffs[0]
    # print(matrix)
    # print("mm:",matrix[:,-1])
    print(matrix)
    return np.linalg.eigvals(matrix)

z = roots(1.0, -2.0, -1.0,2.0) # x^5 + x^4 + x^3 + x^2 + x + 1 = 0
print(z)