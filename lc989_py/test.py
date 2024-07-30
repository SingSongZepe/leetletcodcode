import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, k, expected=None):
        result = Solution().addToArrayForm(nums, k)
        print(result)

    def t1(self, nums, k, expected=None):
        result = Solution1().addToArrayForm(nums, k)
        print(result)

    def test1(self):
        nums = [1, 2, 0, 0]
        k = 34
        self.t(nums, k) # expected: [1, 2, 3, 4]

    def test2(self):
        nums = [2, 7, 4]
        k = 181
        self.t(nums, k) # expected: [4, 5, 5]

    def test3(self):
        nums = [2, 1, 5]
        k = 806
        self.t(nums, k)

    def test4(self):
        nums = [9, 9, 9, 9, 9]
        k = 1
        self.t(nums, k) # expected: [1, 0, 0, 0, 0, 0]

    def test11(self):
        nums = [1, 2, 0, 0]
        k = 34
        self.t1(nums, k) # expected: [1, 2, 3, 4]

    def test21(self):
        nums = [2, 7, 4]
        k = 181
        self.t1(nums, k) # expected: [4, 5, 5]

    def test31(self):
        nums = [2, 1, 5]
        k = 806
        self.t1(nums, k)

    def test41(self):
        nums = [9, 9, 9, 9, 9]
        k = 1
        self.t1(nums, k) # expected: [1, 0, 0, 0, 0, 0]

if __name__ == '__main__':
    unittest.main()