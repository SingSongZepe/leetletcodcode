import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().pivotInteger(arg)
        print(result)

    def t1(self, arg, expected=None):
        result = Solution1().pivotInteger(arg)
        print(result)
        
    def test1(self):
        n = 8
        self.t(n)

    def test2(self):
        n = 1
        self.t(n)

    def test3(self):
        n = 4
        self.t(n)

    def test11(self):
        n = 8
        self.t1(n)

    def test21(self):
        n = 1
        self.t1(n)

    def test31(self):
        n = 4
        self.t1(n)

    def test41(self):
        n = 49
        self.t1(n)



    def test_finder(self):
        to = 10000
        res = finder(to)
        print(res)

if __name__ == '__main__':
    unittest.main()