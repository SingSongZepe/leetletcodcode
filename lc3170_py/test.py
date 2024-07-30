import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().clearStars(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = 'aaba*'
        expected = 'aab'
        self.t(s, expected)

    def test2(self):
        s = 'abc'
        expected = 'abc'
        self.t(s, expected)

    def test3(self):
        s = 'd*yed'
        expected = 'yed'
        self.t(s, expected)

    def test4(self):
        s = 'ed'
        expected = 'ed'
        self.t(s, expected)

    def test5(self):
        s = '**'
        expected = ''


if __name__ == '__main__':
    unittest.main()