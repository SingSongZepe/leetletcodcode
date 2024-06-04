import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, t, expected=None):
        result = Solution().appendCharacters(s, t)
        print(result)

    def t1(self, s, t, expected=None):
        result = Solution1().appendCharacters(s, t)
        print(result)

    def test1(self):
        s = 'coaching'
        t = 'coding'
        self.t(s, t)

    def test2(self):
        s = 'abcde'
        t = 'a'
        self.t(s, t)

    def test3(self):
        s = 'a'
        t = 'z'
        self.t(s, t)

    def test4(self):
        s = 'a'
        t = 'a'
        self.t(s, t)

    def test11(self):
        s = 'coaching'
        t = 'coding'
        self.t1(s, t)

    def test21(self):
        s = 'abcde'
        t = 'a'
        self.t1(s, t)

    def test31(self):
        s = 'a'
        t = 'z'
        self.t1(s, t)

    def test41(self):
        s = 'a'
        t = 'a'
        self.t1(s, t)

    def test_iter(self):
        s = 'abcde'
        it = iter(s)
        if 'b' in it:
            print('b in s')

        for c in it:
            print(c)

if __name__ == '__main__':
    unittest.main()