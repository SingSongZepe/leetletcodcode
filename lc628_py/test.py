import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maximumProduct(arg)
        print(result)
        
    def test1(self):
        nums = [1, 2, 3]
        self.t(nums)

    def test2(self):
        nums = [1, 2, 3, 4]
        self.t(nums)

    def test3(self):
        nums = [-1, -2, -3]
        self.t(nums)

    def test4(self):
        nums = [30, -2, -3, -999]
        self.t(nums)


if __name__ == '__main__':
    unittest.main()