import unittest
from main import *


class Test(unittest.TestCase):

    def t(self, nums, k, expected=None):
        result = Solution().countKDifference(nums, k)
        print(result)

    def t1(self, nums, k, expected=None):
        result = Solution1().countKDifference(nums, k)
        print(result)

    def test1(self):
        nums = [1, 2, 2, 1]
        k = 1
        self.t(nums, k)

    def test2(self):
        nums = [1, 3]
        k = 3
        self.t(nums, k)

    def test3(self):
        nums = [3, 2, 1, 5, 4]
        k = 2
        self.t(nums, k)

    def test11(self):
        nums = [1, 2, 2, 1]
        k = 1
        self.t1(nums, k)

    def test21(self):
        nums = [1, 3]
        k = 3
        self.t1(nums, k)

    def test31(self):
        nums = [3, 2, 1, 5, 4]
        k = 2
        self.t1(nums, k)

if __name__ == '__main__':
    unittest.main()