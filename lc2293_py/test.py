import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums, expected=None):
        result = Solution().minMaxGame(nums)
        print(result)

    def test1(self):
        nums = [1, 3, 5, 2, 4, 8, 2, 2]
        self.t(nums)

    def test2(self):
        nums = [3]
        self.t(nums)

if __name__ == '__main__':
    unittest.main()