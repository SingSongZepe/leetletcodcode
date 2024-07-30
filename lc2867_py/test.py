import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, edges, expected=None):
        result = Solution().countPaths(arg, edges)
        print(result)
        
    def test1(self):
        n = 5
        edges = [[1,2],[1,3],[2,4],[2,5]]
        self.t(n, edges)

    def test2(self):
        n = 6
        edges = [[1, 2], [1, 3], [2, 4], [3, 5], [3, 6]]


if __name__ == '__main__':
    unittest.main()