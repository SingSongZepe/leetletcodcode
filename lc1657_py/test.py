import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, word1, word2, expected=None):
        result = Solution().closeStrings(word1, word2)
        print(result)

    def test1(self):
        word1 = 'abc'
        word2 = 'bca'
        self.t(word1, word2)

    def test2(self):
        word1 = 'a'
        word2 = 'aa'
        self.t(word1, word2)

    def test3(self):
        word1 = 'cabbba'
        word2 = 'abbccc'
        self.t(word1, word2)

if __name__ == '__main__':
    unittest.main()