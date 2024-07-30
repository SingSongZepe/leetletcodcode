import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().findMaxAverage(arg, k)
        print(result)
        
    def test1(self):
        nums = [1, 12, -5, -6, 50, 3]
        k = 4
        self.t(nums, k)

    def test2(self):
        nums = [5]
        k = 1
        self.t(nums, k)


if __name__ == '__main__':
    unittest.main()