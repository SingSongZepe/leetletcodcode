import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().checkString(arg)
        print(result)
        self.assertEqual(result, expected)
    
    def t1(self, arg, expected=None):
        result = Solution().checkString(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = 'aaabbb'
        expected = True
        self.t(s, expected)
    
    def test2(self):
        s = 'abab'
        expected = False
        self.t(s, expected)
    
    def test11(self):
        s = 'aaabbb'
        expected = True
        self.t(s, expected)
    
    def test21(self):
        s = 'abab'
        expected = False
        self.t(s, expected)


if __name__ == '__main__':
    unittest.main()