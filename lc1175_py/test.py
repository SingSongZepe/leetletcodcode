import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, n, expected=None):
        result = Solution().numPrimeArrangements(n)
        print(result)

    def test1(self):
        n = 5
        self.t(n)

    def test2(self):
        n = 10
        self.t(n)

    def test3(self):
        n = 100
        self.t(n)

if __name__ == '__main__':
    unittest.main()