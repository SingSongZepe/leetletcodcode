import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, t, expected=None):
        result = Solution().backspaceCompare(s, t)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, s, t, expected=None):
        result = Solution1().backspaceCompare(s, t)
        print(result)
        self.assertEqual(result, expected)

    # def test1(self):
    #     s = 'ab#c'
    #     t = 'ad#c'
    #     expected = True
    #     self.t(s, t, expected)
    
    # def test2(self):
    #     s = 'ab##'
    #     t = 'c#d#'
    #     expected = True
    #     self.t(s, t, expected)
    
    # def test3(self):
    #     s = 'a#c'
    #     t = 'b'
    #     expected = False
    #     self.t(s, t, expected)
    
    
    def test11(self):
        s = 'ab#c'
        t = 'ad#c'
        expected = True
        self.t1(s, t, expected)
    
    def test21(self):
        s = 'ab##'
        t = 'c#d#'
        expected = True
        self.t1(s, t, expected)
    
    def test31(self):
        s = 'a#c'
        t = 'b'
        expected = False
        self.t1(s, t, expected)
    
    


if __name__ == '__main__':
    unittest.main()