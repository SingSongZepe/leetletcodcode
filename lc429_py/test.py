import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().levelOrder(arg)
        print(result)
    
    def t1(self, arg, expected=None):
        result = Solution1().levelOrder(arg)
        print(result)
        
    def test1(self):
        head = [1, 3, 2, 5, 6, None, 4, None, 1, 7, None]
        root = build_tree(head)
        self.t(root, None)

    def test2(self):
        head = [1, 3, 2, 5, 6, None, 4, None, 1, 7, None]
        root = build_tree(head)
        self.t1(root, None)

if __name__ == '__main__':
    unittest.main()