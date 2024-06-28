import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().evenOddBit(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        n = 50
        self.t(n, [1, 2])

    def test2(self):
        n = 2
        self.t(n, [0, 1])

if __name__ == '__main__':
    unittest.main()