import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().largestTriangleArea(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().largestTriangleArea(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
        self.t(points, 2.00000)

    def test2(self):
        points = [[1,0],[0,0],[0,1]]
        self.t(points, 0.50000)

    def test11(self):
        points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
        self.t1(points, 2.00000)

    def test21(self):
        points = [[1,0],[0,0],[0,1]]
        self.t1(points, 0.50000)


if __name__ == '__main__':
    unittest.main()