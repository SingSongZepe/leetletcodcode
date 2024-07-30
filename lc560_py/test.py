import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, k, expected=None):
        result = Solution().subarraySum(nums, k)
        print(result)

    def test1(self):
        nums = [1, 1, 1]
        k = 2
        self.t(nums, k)

    def test2(self):
        nums = [1, 2, 3]
        k = 3
        self.t(nums, k)

if __name__ == '__main__':
    unittest.main()