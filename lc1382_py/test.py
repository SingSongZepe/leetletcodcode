import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().balanceBST(arg)
        print(result)
        
    def test1(self):
        # replace null with None
        # root = [1,null,2,null,3,null,4,null,null]
        v = [1, None, 2, None, 3, None, 4, None, None]
        root = helper(v)
        self.t(root)

    def test2(self):
        v = [2, 1, 3]
        root = helper(v)
        self.t(root)

if __name__ == '__main__':
    unittest.main()