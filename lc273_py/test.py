import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().numberToWords(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        num = 123
        expected = "One Hundred Twenty Three"
        self.t(num, expected)
    
    def test2(self):
        num = 12345
        expected = "Twelve Thousand Three Hundred Forty Five"
        self.t(num, expected)
    
    def test3(self):
        num = 12000
        expected = "Twelve Thousand"
        self.t(num, expected)
    
    def test4(self):
        num = 20
        expected = "Twenty"
        self.t(num, expected)
    
    def test5(self):
        num = 23
        expected = "Twenty Three"
        self.t(num, expected)

    def test6(self):
        num = 30
        expected = "Thirty"
        self.t(num, expected)
    
    def test7(self):
        num = 100
        expected = "One Hundred"
        self.t(num, expected)

if __name__ == '__main__':
    unittest.main()