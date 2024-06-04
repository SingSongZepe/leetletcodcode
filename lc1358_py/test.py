import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, expected=None):
        result = Solution().numberOfSubstrings(s)
        print(result)

    def t1(self, s, expected=None):
        result = Solution1().numberOfSubstrings(s)
        print(result)

    def test1(self):
        s = 'abcabc'
        self.t(s)

    def test2(self):
        s = 'aaacb'
        self.t(s)

    def test3(self):
        s = 'abc'
        self.t(s)

    def test11(self):
        s = 'abcabc'
        self.t1(s)

    def test21(self):
        s = 'aaacb'
        self.t1(s)

    def test31(self):
        s = 'abc'
        self.t1(s)

if __name__ == '__main__':
    unittest.main()