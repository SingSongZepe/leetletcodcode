import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().sortColors(arg)
        print(result)
        
    def test1(self):
        nums = [2, 0, 2, 1, 1, 0]
        self.t(nums, [0, 0, 1, 1, 2, 2])


    def test2(self):
        nums = [2, 0, 1]
        self.t(nums, [0, 1, 2])

if __name__ == '__main__':
    unittest.main()