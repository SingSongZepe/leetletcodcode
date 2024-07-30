import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maxDepth(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().maxDepth(arg)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        v = [3,9,20,None,None,15,7]
        root = helper(v)
        self.t(root, 3)

    def test2(self):
        v = [1,None,2]
        root = helper(v)
        self.t(root, 2)

    def test3(self):
        v = []
        root = helper(v)
        self.t(root, 0)

    def test11(self):
        v = [3,9,20,None,None,15,7]
        root = helper(v)
        self.t1(root, 3)

    def test21(self):
        v = [1,None,2]
        root = helper(v)
        self.t1(root, 2)

    def test31(self):
        v = []
        root = helper(v)
        self.t1(root, 0)

if __name__ == '__main__':
    unittest.main()