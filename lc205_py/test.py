import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, t, expected=None):
        result = Solution().isIsomorphic(s, t)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = "egg"
        t = "add"
        expected = True
        self.t(s, t, expected)
    
    def test2(self):
        s = "foo"
        t = "bar"
        expected = False
        self.t(s, t, expected)
    
    def test3(self):
        s = "paper"
        t = "title"
        expected = True
        self.t(s, t, expected)
    
    def test4(self):
        s = "badc"
        t = "baba"
        expected = False
        self.t(s, t, expected)

if __name__ == '__main__':
    unittest.main()