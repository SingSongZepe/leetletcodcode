
class Solution:
    def checkRecord(self, n: int) -> int:

        dp = [[[-1] * 3 for _ in range(2)] for _ in range(n+1)]
        def recv(i, a, l) -> int:
            if a > 1 or l > 2:
                return 0
            if i == 0:
                return 1

            if dp[i][a][l] != -1:
                return dp[i][a][l]

            val = recv(i-1, a, 0) # present
            val += recv(i-1, a, l+1) # late
            val += recv(i-1, a+1, 0) # absent

            dp[i][a][l] = val % 1000000007
            return dp[i][a][l]

        return recv(n, 0, 0)


# matrix method
class Solution1:
    def checkRecord(self, n: int) -> int:

        MOD = int(1e9 + 7)

        '''dp0 = [[0 for _ in range(3)] for _ in range(2)]
        dp0[0][0] = 1
        for _ in range(n):
            dp1 = [[0 for _ in range(3)] for _ in range(2)]

            dp1[0][0] = (sum(dp0[0]))%MOD
            dp1[0][1], dp1[0][2] = dp0[0][0], dp0[0][1]
            dp1[1][0] = (sum(dp0[0])+sum(dp0[1]))%MOD
            dp1[1][1], dp1[1][2] = dp0[1][0], dp0[1][1]

            dp0 = dp1.copy()

        return (sum(dp0[0])+sum(dp0[1]))%MOD'''

        def matmul(A, B):
            C = [[0 for _ in range(6)] for _ in range(6)]
            for k in range(6):
                for i in range(6):
                    for j in range(6): C[i][j] = (C[i][j] + A[i][k] * B[k][j]) % MOD
            return C

        def pow(A, n):
            if n == 1: return A

            t = pow(A, int(n / 2))
            t = matmul(t, t)

            if (n % 2) == 0:
                return t
            else:
                return matmul(A, t)

        t = pow([[1, 1, 1, 0, 0, 0],
                 [1, 0, 0, 0, 0, 0],
                 [0, 1, 0, 0, 0, 0],
                 [1, 1, 1, 1, 1, 1],
                 [0, 0, 0, 1, 0, 0],
                 [0, 0, 0, 0, 1, 0]], n)

        return sum([t[i][0] for i in range(6)]) % MOD

def matmul(A, B):
    C = [[0 for _ in range(6)] for _ in range(6)]
    for k in range(6):
        for i in range(6):
            for j in range(6): C[i][j] = (C[i][j] + A[i][k] * B[k][j])
    return C

def pow(A, n):
    if n == 1: return A

    t = pow(A, int(n / 2))
    t = matmul(t, t)

    if (n % 2) == 0:
        return t
    else:
        return matmul(A, t)

def print_matrix(A):
    for i in range(6):
        print(A[i])
    print()

def main():
    print('Hello World')

if __name__ == '__main__':
    main()