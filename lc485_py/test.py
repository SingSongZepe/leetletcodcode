import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().findMaxConsecutiveOnes(arg)
        print(result)
        
    def test1(self):
        nums = [1, 1, 0, 1, 1, 1]
        self.t(nums, 3)

    def test2(self):
        nums = [1, 0, 1, 1, 0, 1]
        self.t(nums, 2)

if __name__ == '__main__':
    unittest.main()