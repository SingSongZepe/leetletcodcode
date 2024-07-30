import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, val, expected=None):
        result = Solution().removeElements(arg, val)
        print_list(result)
        # self.assertEqual(result, expected)
        
    def test1(self):
        v = [1, 2, 6, 3, 4, 5, 6]
        root = helper(v)
        val = 6
        self.t(root, val, helper([1, 2, 3, 4, 5]))

    def test2(self):
        v = []
        root = helper(v)
        val = 1
        self.t(root, val, helper([]))

    def test3(self):
        v = [7, 7, 7, 7]
        root = helper(v)
        val = 7
        self.t(root, val, helper([]))

    def test4(self):
        v = [1, 2]
        root = helper(v)
        val = 1
        self.t(root, val, helper([]))

if __name__ == '__main__':
    unittest.main()