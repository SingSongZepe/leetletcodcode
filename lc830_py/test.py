import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().largeGroupPositions(arg)
        print(result)
        self.assertEqual(result, expected) 
    
    def t1(self, arg, expected=None):
        result = Solution1().largeGroupPositions(arg)
        print(result)
        self.assertEqual(result, expected) 
        
    def test1(self):
        s = "abbxxxxzzy"
        expected = [[3,6]]
        self.t(s, expected)
    
    def test2(self):
        s = "abc"
        expected = []
        self.t(s, expected)
    
    def test3(self):
        s = "abcdddeeeeaabbbcd"
        expected = [[3,5],[6,9],[12,14]]
        self.t(s, expected)
    
    def test11(self):
        s = "abbxxxxzzy"
        expected = [[3,6]]
        self.t1(s, expected)
    
    def test21(self):
        s = "abc"
        expected = []
        self.t1(s, expected)
    
    def test31(self):
        s = "abcdddeeeeaabbbcd"
        expected = [[3,5],[6,9],[12,14]]
        self.t1(s, expected)
    


if __name__ == '__main__':
    unittest.main()