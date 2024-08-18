import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().nthUglyNumber(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        n = 10
        expected = 12
        self.t(n, expected)

    def test2(self):
        n = 1
        expected = 1
        self.t(n, expected)


if __name__ == '__main__':
    unittest.main()