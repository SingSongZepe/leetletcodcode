import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        n = 2
        result = Solution().checkRecord(n)
        print(result)

    def test2(self):
        n = 1
        result = Solution().checkRecord(n)
        print(result)

    def test3(self):
        n = 3
        result = Solution().checkRecord(n)
        print(result)

    def test4(self):
        n = 10101
        result = Solution().checkRecord(n)
        print(result)


    def test11(self):
        n = 2
        result = Solution1().checkRecord(n)
        print(result)

    def test21(self):
        n = 1
        result = Solution1().checkRecord(n)
        print(result)

    def test31(self):
        n = 3
        result = Solution1().checkRecord(n)
        print(result)

    def test41(self):
        n = 10101
        result = Solution1().checkRecord(n)
        print(result)

    def test_pow_n(self):
        n = 2
        A = [[1, 1, 1, 0, 0, 0],
             [1, 0, 0, 0, 0, 0],
             [0, 1, 0, 0, 0, 0],
             [1, 1, 1, 1, 1, 1],
             [0, 0, 0, 1, 0, 0],
             [0, 0, 0, 0, 1, 0]]

        result = pow(A, n)
        print_matrix(result)

if __name__ == '__main__':
    unittest.main()