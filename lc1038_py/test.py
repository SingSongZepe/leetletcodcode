import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().bstToGst(arg)
        print(result)
        
    def test1(self):
        v = [4,1,6,0,2,5,7,None,None,None,3,None,None,None,8]
        root = helper(v)
        self.t(root)

    def test2(self):
        v = [0,None,1]
        root = helper(v)
        self.t(root)

if __name__ == '__main__':
    unittest.main()