import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().maximumDifference(nums)
        print(result)

    def test1(self):
        nums = [7, 1, 5, 4]
        self.t(nums) # expected output: 4

    def test2(self):
        nums = [9, 4, 3, 2]
        self.t(nums) # expected output: -1

    def test3(self):
        nums = [1, 5, 2, 10]
        self.t(nums) # expected output: 9

    def test4(self):
        nums = [87,68,91,86,58,63,43,98,6,40]
        self.t(nums) # expected output: 55

if __name__ == '__main__':
    unittest.main()