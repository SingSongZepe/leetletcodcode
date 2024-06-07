import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().findTarget(arg, k)
        print(result)
        
    def test1(self):
        nums = [5, 3, 6, 2, 4, -1, 7]
        k = 9
        self.t(nums, k)

    def test2(self):
        nums = [5, 3, 6, 2, 4 ,-1, 7]
        k = 28
        self.t(nums, k)

if __name__ == '__main__':
    unittest.main()