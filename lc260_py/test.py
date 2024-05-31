import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().singleNumber(nums)
        print(result)

    def test1(self):
        nums = [1, 2, 1, 3, 2, 5]
        self.t(nums)

    def test2(self):
        nums = [-1, 0]
        self.t(nums)

    def test3(self):
        nums = [0, 1]
        self.t(nums)


if __name__ == '__main__':
    unittest.main()