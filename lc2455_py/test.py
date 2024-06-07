import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().averageValue(arg)
        print(result)
        
    def test1(self):
        nums = [1, 3, 6, 10, 12, 15]
        self.t(nums)

    def test2(self):
        nums = [1, 2, 4, 7, 10]
        self.t(nums)


if __name__ == '__main__':
    unittest.main()