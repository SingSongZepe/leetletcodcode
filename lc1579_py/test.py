import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, edges, expected=None):
        result = Solution().maxNumEdgesToRemove(arg, edges)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        n = 4
        edges = [[3, 1, 2], [3, 2, 3], [1, 1, 3], [1, 2, 4], [1, 1, 2], [2, 3, 4]]
        expected = 2
        self.t(n, edges, expected)

    def test2(self):
        n = 4
        edges = [[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]
        expected = 0
        self.t(n, edges, expected)

    def test3(self):
        n = 4
        edges = [[3,2,3],[1,1,2],[2,3,4]]
        expected = -1
        self.t(n, edges, expected)



if __name__ == '__main__':
    unittest.main()