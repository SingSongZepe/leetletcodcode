import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, k, rc, cc, expected=None):
        result = Solution().buildMatrix(k, rc, cc)
        print(result)
        
    def test1(self):
        k = 3
        rowConditions = [[1,2],[3,2]]
        colConditions = [[2,1],[3,2]]
        self.t(k, rowConditions, colConditions)


if __name__ == '__main__':
    unittest.main()