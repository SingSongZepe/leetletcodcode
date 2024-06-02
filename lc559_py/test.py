import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, root, expected=None):
        result = Solution().maxDepth(root)
        print(result)

    def test1(self):
        v = [1, NE_INFINITE_, 3, 2, 4, NE_INFINITE_, 5, 6]
        root = helper(v)
        self.t(root)

    def test2(self):
        # replace all null with NE_INFINITE_
        # v = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
        v = [1, NE_INFINITE_, 2, 3, 4, 5, NE_INFINITE_, NE_INFINITE_, 6, 7, NE_INFINITE_, 8, NE_INFINITE_, 9, 10, NE_INFINITE_, NE_INFINITE_, 11, NE_INFINITE_, 12, NE_INFINITE_, 13, NE_INFINITE_, NE_INFINITE_, 14]
        root = helper(v)
        self.t(root)


    def test_helper(self):
        v = [1, NE_INFINITE_, 3, 2, 4, NE_INFINITE_, 5, 6]
        root = helper(v)
        print(root.val)
        print(root.children[0].val)
        print(root.children[1].val)
        print(root.children[2].val)
        print(root.children[0].children[0].val)
        print(root.children[0].children[1].val)

if __name__ == '__main__':
    unittest.main()