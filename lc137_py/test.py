import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [2, 2, 3, 2]
        result = Solution().singleNumber(nums)
        print(result) # Output: 3

    def test2(self):
        nums = [0, 1, 0, 1, 0, 1, 99]
        result = Solution().singleNumber(nums)
        print(result) # Output: 99

    def test11(self):
        nums = [2, 2, 3, 2]
        result = Solution1().singleNumber(nums)
        print(result) # Output: 3

    def test21(self):
        nums = [0, 1, 0, 1, 0, 1, 99]
        result = Solution1().singleNumber(nums)
        print(result) # Output: 99

    def test31(self):
        nums = [3, 3, 3, 3]
        result = Solution1().singleNumber(nums)
        print(result) # Output: 3

    def test41(self):
        nums = [3, 3, 3]
        result = Solution1().singleNumber(nums)
        print(result) # Output: 0

if __name__ == '__main__':
    unittest.main()