import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().stringMatching(arg)
        print(result)
        # self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().stringMatching(arg)
        print(result)
        # self.assertEqual(result, expected)

    def t2(self, arg, expected=None):
        result = Solution2().stringMatching(arg)
        print(result)
        # self.assertEqual(result, expected)
        
    def test1(self):
        words = ["mass","as","hero","superhero"]
        self.t(words)

    def test2(self):
        words = ["leetcode","et","code"]
        self.t(words)

    def test11(self):
        words = ["mass","as","hero","superhero"]
        self.t1(words)

    def test21(self):
        words = ["leetcode","et","code"]
        self.t1(words)

    def test12(self):
        words = ["mass","as","hero","superhero"]
        self.t2(words)

    def test22(self):
        words = ["leetcode","et","code"]
        self.t2(words)

if __name__ == '__main__':
    unittest.main()