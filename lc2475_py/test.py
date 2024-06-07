import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().unequalTriplets(arg)
        print(result)

    def t1(self, arg, expected=None):
        result = Solution1().unequalTriplets(arg)
        print(result)
        
    def test1(self):
        nums = [4, 4, 2, 4, 3]
        self.t(nums)

    def test2(self):
        nums = [1, 1, 1, 1, 1]
        self.t(nums)

    def test3(self):
        nums = [5, 4, 4, 2, 4, 3]
        self.t(nums)

    def test11(self):
        nums = [4, 4, 2, 4, 3]
        self.t1(nums)

    def test21(self):
        nums = [1, 1, 1, 1, 1]
        self.t1(nums)

    def test31(self):
        nums = [5, 4, 4, 2, 4, 3]
        self.t1(nums)

if __name__ == '__main__':
    unittest.main()