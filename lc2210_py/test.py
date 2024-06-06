import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().countHillValley(arg)
        print(result)
        
    def test1(self):
        nums = [2, 4, 1, 1, 6, 5]
        self.t(nums)

    def test2(self):
        nums = [6, 6, 5, 5, 4, 1]
        self.t(nums)

    def test3(self):
        nums = [5,7,7,1,7]
        self.t(nums)

if __name__ == '__main__':
    unittest.main()