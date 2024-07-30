import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().addDigits(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        num = 38
        excepted = 2
        self.t(num, excepted)
        
    def test2(self):
        num = 0
        excepted = 0
        self.t(num, excepted)

if __name__ == '__main__':
    unittest.main()