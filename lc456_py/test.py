import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [1, 2, 3, 4]
        result = Solution().find132pattern(nums)
        print(result)

    def test2(self):
        nums = [3, 1, 4, 2]
        result = Solution().find132pattern(nums)
        print(result)

    def test3(self):
        nums = [-1, 3, 2, 0]
        result = Solution().find132pattern(nums)
        print(result)

    def test4(self):
        nums = [-2, 1, -1]
        result = Solution().find132pattern(nums)
        print(result)


    def test11(self):
        nums = [1, 2, 3, 4]
        result = Solution1().find132pattern(nums)
        print(result)

    def test21(self):
        nums = [3, 1, 4, 2]
        result = Solution1().find132pattern(nums)
        print(result)

    def test31(self):
        nums = [-1, 3, 2, 0]
        result = Solution1().find132pattern(nums)
        print(result)

    def test41(self):
        nums = [-2, 1, -1]
        result = Solution1().find132pattern(nums)
        print(result)

    # write a test case with 40 length nums
    def test51(self):
        nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 50, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]
        result = Solution1().find132pattern(nums)
        print(result)


if __name__ == '__main__':
    unittest.main()