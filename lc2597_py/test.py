import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [2, 4, 6]
        k = 2
        result = Solution().beautifulSubsets(nums, k)
        print(result)

    def test2(self):
        nums = [1]
        k = 1
        result = Solution().beautifulSubsets(nums, k)
        print(result)



    def test_sort(self):
        nums = [1, 5, 3, 4, 2]
        nums.sort()
        print(nums)



if __name__ == '__main__':
    unittest.main()