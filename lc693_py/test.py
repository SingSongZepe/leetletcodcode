import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().hasAlternatingBits(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().hasAlternatingBits(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        n = 5
        self.t(n, True)

    def test2(self):
        n = 7
        self.t(n, False)

    def test3(self):
        n = 11
        self.t(n, False)

    def test4(self):
        n = 21
        self.t(n, True)

    def test5(self):
        n = 10
        self.t(n, True)

    def test41(self):
        n = 21
        self.t1(n, True)

    def test51(self):
        n = 10
        self.t1(n, True)

if __name__ == '__main__':
    unittest.main()