import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, original, m, n, expected=None):
        result = Solution().construct2DArray(original, m, n)
        print(result)

    def test1(self):
        original = [1, 2, 3, 4]
        m = 2
        n = 2
        self.t(original, m, n)

    def test2(self):
        original = [1, 2, 3]
        m = 1
        n = 3
        self.t(original, m, n)

    def test3(self):
        original = [1, 2]
        m = 1
        n = 1
        self.t(original, m, n)

if __name__ == '__main__':
    unittest.main()