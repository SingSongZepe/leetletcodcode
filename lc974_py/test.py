import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().subarraysDivByK(arg, k)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, k, expected=None):
        result = Solution1().subarraysDivByK(arg, k)
        print(result)
        self.assertEqual(result, expected)

    def test1(self):
        nums = [4, 5, 0, -2, -3, 1]
        k = 5
        self.t(nums, k, 7)

    def test2(self):
        nums = [5]
        k = 9
        self.t(nums, k, 0)

    def test11(self):
        nums = [4, 5, 0, -2, -3, 1]
        k = 5
        self.t1(nums, k, 7)

    def test21(self):
        nums = [5]
        k = 9
        self.t1(nums, k, 0)


if __name__ == '__main__':
    unittest.main()