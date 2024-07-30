import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().minimumDeletions(arg)
        print(result)
        self.assertEqual(result, expected)
    
    def t1(self, arg, expected=None):
        result = Solution1().minimumDeletions(arg)
        print(result)
        self.assertEqual(result, expected)
        
    # def test1(self):
    #     s = 'aababbab'
    #     expected = 2
    #     self.t(s, expected)
    
    # def test2(self):
    #     s = 'bbaaaaabb'
    #     expected = 2
    #     self.t(s, expected)
    
    # def test3(self):
    #     s = 'aaaaaa'
    #     expected = 0
    #     self.t(s, expected)    

    # def test4(self):
    #     s = 'bbbbbbbbbb'
    #     expected = 0
    #     self.t(s, expected)
            
    def test11(self):
        s = 'aababbab'
        expected = 2
        self.t1(s, expected)
    
    def test21(self):
        s = 'bbaaaaabb'
        expected = 2
        self.t1(s, expected)

if __name__ == '__main__':
    unittest.main()