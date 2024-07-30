import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        words = ["abc","aabc","bc"]
        result = Solution().makeEqual(words)
        print(result) # Output: True

    def test2(self):
        words = ["ab", "a"]
        result = Solution().makeEqual(words)
        print(result) # Output: False

if __name__ == '__main__':
    unittest.main()