import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, t, expected=None):
        result = Solution().minSteps(s, t)
        print(result)

    def t2(self, s, t, expected=None):
        result = Solution2().minSteps(s, t)
        print(result)

    def test1(self):
        s = 'bab'
        t = 'aba'
        self.t(s, t)

    def test2(self):
        s = 'leetcode'
        t = 'practice'
        self.t(s, t)

    def test3(self):
        s = 'anagram'
        t = 'nagaram'
        self.t(s, t)

    def test4(self):
        s = 'xxyyzz'
        t = 'xxyyzz'
        self.t(s, t)

    def test5(self):
        s = 'friend'
        t = 'family'
        self.t(s, t)

    def test12(self):
        s = 'bab'
        t = 'aba'
        self.t2(s, t)

    def test52(self):
        s = 'friend'
        t = 'family'
        self.t2(s, t)


if __name__ == '__main__':
    unittest.main()