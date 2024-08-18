import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().reverseList(arg)
        print_list(result)
        
    def test1(self):
        head = build_list([1, 2, 3, 4, 5])
        self.t(head)
    
    def test2(self):
        head = build_list([1, 2])
        self.t(head)
    
    def test3(self):
        head = build_list([])
        self.t(head)

if __name__ == '__main__':
    unittest.main()