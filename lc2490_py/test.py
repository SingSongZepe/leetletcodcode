import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().isCircularSentence(arg)
        print(result)
        
    def test1(self):
        sentence = "leetcode exercises sound delightful"
        self.t(sentence)

    def test2(self):
        sentence = "eetcode"
        self.t(sentence)

    def test3(self):
        sentence = "Leetcode is cool"
        self.t(sentence)

    def test4(self):
        sentence = "leetcode"
        self.t(sentence)


if __name__ == '__main__':
    unittest.main()