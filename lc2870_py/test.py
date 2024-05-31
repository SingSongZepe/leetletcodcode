import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().minOperations(nums)
        print(result)

    def t1(self, nums, expected=None):
        result = Solution1().minOperations(nums)
        print(result)

    def test1(self):
        nums = [2,3,3,2,2,4,2,3,4]
        self.t(nums)

    def test2(self):
        nums = [2, 1, 2, 2, 3, 3]
        self.t(nums)

    def test3(self):
        nums = [14,12,14,14,12,14,14,12,12,12,12,14,14,12,14,14,14,12,12]
        self.t(nums)

    def test4(self):
        nums = [19,19,19,19,19,19,19,19,19,19,19,19,19]
        self.t(nums)


    def test11(self):
        nums = [2,3,3,2,2,4,2,3,4]
        self.t1(nums)

    def test21(self):
        nums = [2, 1, 2, 2, 3, 3]
        self.t1(nums)

    def test31(self):
        nums = [14,12,14,14,12,14,14,12,12,12,12,14,14,12,14,14,14,12,12]
        self.t1(nums)

    def test41(self):
        nums = [19,19,19,19,19,19,19,19,19,19,19,19,19]
        self.t1(nums)

if __name__ == '__main__':
    unittest.main()