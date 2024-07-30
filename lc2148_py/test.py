import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [11, 7, 2, 15]
        result = Solution().countElements(nums)
        print(result)

    def test2(self):
        nums = [-3,3,3,90]
        result = Solution().countElements(nums)
        print(result)

    def test3(self):
        nums = [-3,-3,3,90]
        result = Solution().countElements(nums)
        print(result)

    def test4(self):
        nums = [723,723,-423,723,-647,532,723,723,212,-391,723]
        result = Solution().countElements(nums)
        print(result)

    def test5(self):
        nums = [22206,42301,42301,42301,22206,22206,42301,22206,42301,42301,42301,22206,42301,22206,42301,22206,22206,22206,42301,22206,42301,42301,42301,22206,22206,42301,42301,42301,22206,22206,42301,22206,22206,42301,22206,22206,42301,42301,22206,42301,22206,42301,42301,42301,22206,22206,22206,22206,42301,42301,42301,42301,22206,42301,22206,42301,22206,22206,42301,42301,22206,22206,22206,22206,42301,22206,22206]
        result = Solution().countElements(nums)
        print(result)

    def test6(self):
        nums = [0]
        result = Solution().countElements(nums)
        print(result)

    def test7(self):
        nums = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        result = Solution().countElements(nums)
        print(result)

    def test51(self):
        nums = [22206,42301,42301,42301,22206,22206,42301,22206,42301,42301,42301,22206,42301,22206,42301,22206,22206,22206,42301,22206,42301,42301,42301,22206,22206,42301,42301,42301,22206,22206,42301,22206,22206,42301,22206,22206,42301,42301,22206,42301,22206,42301,42301,42301,22206,22206,22206,22206,42301,42301,42301,42301,22206,42301,22206,42301,22206,22206,42301,42301,22206,22206,22206,22206,42301,22206,22206]
        result = Solution1().countElements(nums)
        print(result)

    def test61(self):
        nums = [0]
        result = Solution1().countElements(nums)
        print(result)

    def test71(self):
        nums = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        result = Solution1().countElements(nums)
        print(result)

if __name__ == '__main__':
    unittest.main()