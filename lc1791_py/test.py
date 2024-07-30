import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().findCenter(arg)
        print(result)
        
    def test1(self):
        edges = [[1,2],[2,3],[4,2]]
        expected = 2
        self.t(edges, expected)

    def test2(self):
        edges = [[1,2],[5,1],[1,3],[1,4]]
        expected = 1
        self.t(edges, expected)

if __name__ == '__main__':
    unittest.main()