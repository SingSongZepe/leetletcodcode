import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().isSymmetric(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().isSymmetric(arg)
        print(result)
        self.assertEqual(result, expected)

    def t2(self, arg, expected=None):
        result = Solution2().isSymmetric(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        v = [1, 2, 2, 3, 4, 4, 3]
        root = helper(v)
        self.t(root, True)

    def test2(self):
        v = [1, 2, 2, None, 3, None, 3]
        root = helper(v)
        self.t(root, False)

    def test3(self):
        v = [1,2,2,3,None,-1,3]
        root = helper(v)
        self.t(root, False)

    def test11(self):
        v = [1, 2, 2, 3, 4, 4, 3]
        root = helper(v)
        self.t1(root, True)

    def test21(self):
        v = [1, 2, 2, None, 3, None, 3]
        root = helper(v)
        self.t1(root, False)

    def test31(self):
        v = [1,2,2,3,None,-1,3]
        root = helper(v)
        self.t1(root, False)

    def test22(self):
        v = [1, 2, 2, None, 3, None, 3]
        root = helper(v)
        self.t2(root, False)

    def test32(self):
        v = [1,2,2,3,None,-1,3]
        root = helper(v)
        self.t2(root, False)

if __name__ == '__main__':
    unittest.main()