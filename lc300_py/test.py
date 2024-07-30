import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().lengthOfLIS(nums)
        print(result)

    def t1(self, nums, expected=None):
        result = Solution1().lengthOfLIS(nums)
        print(result)

    def t2(self, nums, expected=None):
        result = Solution2().lengthOfLIS(nums)
        print(result)

    def test1(self):
        nums = [10,9,2,5,3,7,101,18]
        self.t(nums)

    def test2(self):
        nums = [0, 1, 0, 3, 2, 3]
        self.t(nums)

    def test3(self):
        nums = [7, 7, 7, 7, 7, 7, 7]
        self.t(nums)

    def test11(self):
        nums = [10,9,2,5,3,7,101,18]
        self.t1(nums)

    def test21(self):
        nums = [0, 1, 0, 3, 2, 3]
        self.t1(nums)

    def test31(self):
        nums = [7, 7, 7, 7, 7, 7, 7]
        self.t1(nums)

    def test12(self):
        nums = [10,9,2,5,3,7,101,18]
        self.t2(nums)

    def test22(self):
        nums = [0, 1, 0, 3, 2, 3]
        self.t2(nums)

    def test32(self):
        nums = [7, 7, 7, 7, 7, 7, 7]
        self.t2(nums)

    def test_bisect(self):
        nums = [10,9,2,5,3,7,101,18]
        nums.sort()

        print(nums)
        print(bisect.bisect_left(nums, 19))
        print(bisect.bisect_left(nums, 10))

if __name__ == '__main__':
    unittest.main()