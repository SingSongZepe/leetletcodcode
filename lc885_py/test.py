import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, rows, cols, rs, cs, expected=None):
        result = Solution().spiralMatrixIII(rows, cols, rs, cs)
        print(result)
        
    def test1(self):
        rows = 1
        cols = 4
        rs = 0
        cs = 0
        expected = [[0, 0], [0, 1], [0, 2], [0, 3]]
        self.t(rows, cols, rs, cs, expected)
        
    def test2(self):
        rows = 5
        cols = 6
        rs = 1
        cs = 4
        expected = [[1, 4], [1, 5], [2, 5], [2, 4], [2, 3], [1, 3], [0, 3], [0, 4], [0, 5], [3, 5], [3, 4], [3, 3], [3, 2], [2, 2], [1, 2], [0, 2], [4, 5], [4, 4], [4, 3], [4, 2], [4, 1], [3, 1], [2, 1], [1, 1], [0, 1], [4, 0], [3, 0], [2, 0], [1, 0], [0, 0]]
        self.t(rows, cols, rs, cs, expected)
        

if __name__ == '__main__':
    unittest.main()