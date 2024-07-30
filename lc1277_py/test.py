import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        matrix = [
            [0, 1, 1, 1],
            [1, 1, 1, 1],
            [0, 1, 1, 1],
        ]
        result = Solution().countSquares(matrix)
        print(result) # expected output: 15

    def test2(self):
        matrix = [
            [1, 0, 1],
            [1, 1, 0],
            [1, 1, 0],
        ]
        result = Solution().countSquares(matrix)
        print(result) # expected output: 7

if __name__ == '__main__':
    unittest.main()