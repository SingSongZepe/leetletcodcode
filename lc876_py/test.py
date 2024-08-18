import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().middleNode(arg)
        print_list(result)
        
    def test1(self):
        l = [1, 2, 3, 4, 5]
        li = build_list(l)
        self.t(li)
    
    def test2(self):
        l = [1, 2, 3, 4, 5, 6]
        li = build_list(l)
        self.t(li)
    
    def test3(self):
        l = [1, 2, 3, 4, 5, 6, 7]
        li = build_list(l)
        self.t(li)
    
    def test4(self):
        l = [1, 2, 3, 4, 5, 6, 7, 8]
        li = build_list(l)
        self.t(li)
    


if __name__ == '__main__':
    unittest.main()