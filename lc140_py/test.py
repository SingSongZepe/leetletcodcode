import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = 'catsanddog'
        wordDict = ['cat', 'cats', 'and', 'sand', 'dog']
        result = Solution().wordBreak(s, wordDict)
        print(result)

    def test2(self):
        s = 'pineapplepenapple'
        wordDict = ['apple', 'pen', 'applepen', 'pine', 'pineapple']
        result = Solution().wordBreak(s, wordDict)
        print(result)

    def test3(self):
        s = 'catsandog'
        wordDict = ['cat', 'cats', 'and', 'sand', 'dog']
        result = Solution().wordBreak(s, wordDict)
        print(result)

if __name__ == '__main__':
    unittest.main()