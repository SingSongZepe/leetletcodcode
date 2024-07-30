import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, words, chars, expected=None):
        result = Solution().countCharacters(words, chars)
        print(result)

    def t1(self, words, chars, expected=None):
        result = Solution1().countCharacters(words, chars)
        print(result)

    def test1(self):
        words = ["cat", "bt", "hat", "tree"]
        chars = "atach"
        self.t(words, chars)

    def test2(self):
        words = ["hello", "world", "leetcode"]
        chars = "welldonehoneyr"
        self.t(words, chars)

    def test11(self):
        words = ["cat", "bt", "hat", "tree"]
        chars = "atach"
        self.t1(words, chars)

    def test21(self):
        words = ["hello", "world", "leetcode"]
        chars = "welldonehoneyr"
        self.t1(words, chars)

if __name__ == '__main__':
    unittest.main()