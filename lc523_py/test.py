import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().checkSubarraySum(arg, k)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, k, expected=None):
        result = Solution1().checkSubarraySum(arg, k)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        nums = [23, 2, 4, 6, 7]
        k = 6
        self.t(nums, k, expected=True)

    def test2(self):
        nums = [23, 2, 6, 4, 7]
        k = 6
        self.t(nums, k, expected=True)

    def test3(self):
        nums = [23, 2, 6, 4, 7]
        k = 13
        self.t(nums, k, expected=False)

    def test4(self):
        nums = [97, 6, 13]
        k = 6
        self.t(nums, k, expected=False)

    def test5(self):
        nums = [5, 0, 0, 0]
        k = 3
        self.t(nums, k, expected=True)

    def test6(self):
        nums = [23, 6, 9]
        k = 6
        self.t(nums, k, expected=False)

    def test7(self):
        nums = [23,2,4,6,6]
        k = 7
        self.t(nums, k, expected=True)

    def test11(self):
        nums = [23, 2, 4, 6, 7]
        k = 6
        self.t1(nums, k, expected=True)

    def test51(self):
        nums = [5, 0, 0, 0]
        k = 3
        self.t1(nums, k, expected=True)

    def test61(self):
        nums = [23, 6, 9]
        k = 6
        self.t1(nums, k, expected=False)

if __name__ == '__main__':
    unittest.main()