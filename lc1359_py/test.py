import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, n, expected=None):
        result = Solution().countOrders(n)
        print(result)

    def test1(self):
        n = 1
        self.t(n)

    def test2(self):
        n = 2
        self.t(n)

    def test3(self):
        n = 3
        self.t(n)

if __name__ == '__main__':
    unittest.main()