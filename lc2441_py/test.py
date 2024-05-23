import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [-1, 2, -3, 3]
        result = Solution().findMaxK(nums)
        print(result)

    def test2(self):
        nums = [-1, 10, 6, 7, -7, 1]
        result = Solution().findMaxK(nums)
        print(result)

    def test3(self):
        nums = [-10, 8, 6, 7, -2, -3]
        result = Solution().findMaxK(nums)
        print(result)

    # Solution 1
    def test11(self):
        nums = [-1, 2, -3, 3]
        result = Solution1().findMaxK(nums)
        print(result)

    def test21(self):
        nums = [-1, 10, 6, 7, -7, 1]
        result = Solution1().findMaxK(nums)
        print(result)

    def test31(self):
        nums = [-10, 8, 6, 7, -2, -3]
        result = Solution1().findMaxK(nums)
        print(result)


if __name__ == '__main__':
    unittest.main()