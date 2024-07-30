import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        mat = [[1, 2, 3],
               [4, 5, 6],
               [7, 8, 9]]
        result = Solution().diagonalSum(mat)
        print(result)

    def test2(self):
        mat = [[1, 1, 1, 1],
               [1, 1, 1, 1],
               [1, 1, 1, 1],
               [1, 1, 1, 1]]
        result = Solution().diagonalSum(mat)
        print(result)

    def test3(self):
        mat = [[5]]
        result = Solution().diagonalSum(mat)
        print(result)


    def test11(self):
        mat = [[1, 2, 3],
               [4, 5, 6],
               [7, 8, 9]]
        result = Solution1().diagonalSum(mat)
        print(result)

    def test21(self):
        mat = [[1, 1, 1, 1],
               [1, 1, 1, 1],
               [1, 1, 1, 1],
               [1, 1, 1, 1]]
        result = Solution1().diagonalSum(mat)
        print(result)

    def test31(self):
        mat = [[5]]
        result = Solution1().diagonalSum(mat)
        print(result)

if __name__ == '__main__':
    unittest.main()