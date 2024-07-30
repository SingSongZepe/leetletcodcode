import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, expected=None):
        result = Solution().scoreOfString(s)
        print(result)

    def test1(self):
        s = 'hello'
        self.t(s)

    def test2(self):
        s = 'zaz'
        self.t(s)


if __name__ == '__main__':
    unittest.main()