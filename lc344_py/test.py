import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, expected=None):
        Solution().reverseString(s)
        print(s)

    def t1(self, s, expected=None):
        Solution1().reverseString(s)
        print(s)

    def t2(self, s, expected=None):
        Solution2().reverseString(s)
        print(s)

    def test1(self):
        s = ["h","e","l","l","o"]
        self.t(s)

    def test2(self):
        s = ["H","a","n","n","a","h"]
        self.t(s)

    def test11(self):
        s = ["h","e","l","l","o"]
        self.t1(s)

    def test21(self):
        s = ["H","a","n","n","a","h"]
        self.t1(s)

    def test12(self):
        s = ["h","e","l","l","o"]
        self.t2(s)

    def test22(self):
        s = ["H","a","n","n","a","h"]
        self.t2(s)


if __name__ == '__main__':
    unittest.main()