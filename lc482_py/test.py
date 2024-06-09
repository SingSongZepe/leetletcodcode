import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().licenseKeyFormatting(arg, k)
        print(result)

    def t1(self, arg, k, expected=None):
        result = Solution1().licenseKeyFormatting(arg, k)
        print(result)

    def test1(self):
        s = "5F3Z-2e-9-w"
        k = 4
        self.t(s, k)

    def test2(self):
        s = "2-5g-3-J"
        k = 2
        self.t(s, k)

    def test11(self):
        s = "5F3Z-2e-9-w"
        k = 4
        self.t1(s, k)

    def test21(self):
        s = "2-5g-3-J"
        k = 2
        self.t1(s, k)


if __name__ == '__main__':
    unittest.main()