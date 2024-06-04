import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().sortedSquares(nums)
        print(result)

    def test1(self):
        nums = [-4,-1,0,3,10]
        self.t(nums) # output: [0,1,9,16,100]

    def test2(self):
        nums = [-7,-3,2,3,11]
        self.t(nums) # output: [4,9,9,49,121]


if __name__ == '__main__':
    unittest.main()