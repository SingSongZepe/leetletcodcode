import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, root1, root2, expected=None):
        root = Solution().mergeTrees(root1, root2)
        print(root)

    def t1(self, root1, root2, expected=None):
        root = Solution1().mergeTrees(root1, root2)
        print(root)

    def test1(self):
        v1 = [1,3,2,5]
        v2 = [2,1,3,NE_INFINITY_,4,NE_INFINITY_,7]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t(root1, root2)

    def test11(self):
        v1 = [1,3,2,5]
        v2 = [2,1,3,NE_INFINITY_,4,NE_INFINITY_,7]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t1(root1, root2)

    def test_helper(self):
        v = [1, 3, 2, 5]
        root = helper(v)
        print(root.val)
        print(root.left.val)
        print(root.right.val)
        print(root.left.left.val)

if __name__ == '__main__':
    unittest.main()