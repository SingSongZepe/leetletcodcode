import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, goal, expected=None):
        result = Solution().minBitFlips(arg, goal)
        print(result)

    def t1(self, arg, goal, expected=None):
        result = Solution1().minBitFlips(arg, goal)
        print(result)

    def test1(self):
        start = 10
        goal = 7
        self.t(start, goal)

    def test2(self):
        start = 3
        goal = 4
        self.t(start, goal)

    def test11(self):
        start = 10
        goal = 7
        self.t1(start, goal)

    def test21(self):
        start = 3
        goal = 4
        self.t1(start, goal)

if __name__ == '__main__':
    unittest.main()