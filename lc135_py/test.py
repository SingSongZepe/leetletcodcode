import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        ratings = [1, 0, 2]
        result = Solution().candy(ratings)
        print(result) # expected output: 5

    def test2(self):
        ratings = [1, 2, 2]
        result = Solution().candy(ratings)
        print(result) # expected output: 4


if __name__ == '__main__':
    unittest.main()