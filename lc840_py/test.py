import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().numMagicSquaresInside(arg)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]]
        expected = 1
        self.t(grid, expected)
    
    def test2(self):
        grid = [[8]]
        expected = 0
        self.t(grid, expected)

if __name__ == '__main__':
    unittest.main()