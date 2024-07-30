import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().distinctAverages(arg)
        print(result)
        
    def test1(self):
        nums = [4, 1, 4, 0, 3, 5]
        self.t(nums)

    def test2(self):
        nums = [1, 100]
        self.t(nums)

if __name__ == '__main__':
    unittest.main()