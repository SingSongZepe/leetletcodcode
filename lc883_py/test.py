import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().projectionArea(arg)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        grid = [[1, 2], [3, 4]]
        expected = 17
        self.t(grid, expected)
    
    def test2(self):
        grid = [[2]]
        expected = 5
        self.t(grid, expected)

    def test3(self):
        grid = [[1, 0], [0, 2]]
        expected = 8
        self.t(grid, expected)



if __name__ == '__main__':
    unittest.main()