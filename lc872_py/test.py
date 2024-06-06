import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, root2, expected=None):
        result = Solution().leafSimilar(arg, root2)
        print(result)

    def t1(self, arg, root2, expected=None):
        result = Solution1().leafSimilar(arg, root2)
        print(result)
        
    def test1(self):
        v1 = [3, 5, 1, 6, 2, 9, 8, -1, -1, 7, 4]
        v2 = [3, 5, 1, 6, 7, 4, 2, -1, -1, -1, -1, -1, -1, 9, 8]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t(root1, root2)

    def test2(self):
        v1 = [1, 2, 3]
        v2 = [1, 3, 2]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t(root1, root2)

    def test3(self):
        v1 = [3,5,1,6,7,4,2,-1,-1,-1,-1,-1,-1,9,11,-1,-1,8,10]
        v2 = [3,5,1,6,2,9,8,-1,-1,7,4]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t(root1, root2)


    def test11(self):
        v1 = [3, 5, 1, 6, 2, 9, 8, -1, -1, 7, 4]
        v2 = [3, 5, 1, 6, 7, 4, 2, -1, -1, -1, -1, -1, -1, 9, 8]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t1(root1, root2)

    def test21(self):
        v1 = [1, 2, 3]
        v2 = [1, 3, 2]
        root1 = helper(v1)
        root2 = helper(v2)
        self.t1(root1, root2)


    def test_helper(self):
        v1 = [3,5,1,6,2,9,8,-1,-1,7,4]
        v2 = [3,5,1,6,7,4,2,-1,-1,-1,-1,-1,-1,9,8]
        root1 = helper(v1)
        root2 = helper(v2)
        print(root1.val)
        print(root2.val)

if __name__ == '__main__':
    unittest.main()