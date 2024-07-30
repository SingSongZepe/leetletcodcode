import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [1, 2, 3, 6]
        result = Solution().countQuadruplets(nums)
        print(result)

    def test2(self):
        nums = [3,3,6,4,5]
        result = Solution().countQuadruplets(nums)
        print(result)

    def test2(self):
        nums = [1,1,1,3,5]
        result = Solution().countQuadruplets(nums)
        print(result)

if __name__ == '__main__':
    unittest.main()