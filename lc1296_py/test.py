import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, k, expected=None):
        result = Solution().isPossibleDivide(arg, k)
        print(result)
        
    def test1(self):
        nums = [1, 2, 3, 3, 4, 4, 5, 6]
        k = 4
        self.t(nums, k)

    def test2(self):
        nums = [3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11]
        k = 3
        self.t(nums, k)

    def test3(self):
        nums = [1, 2, 3, 4]
        k = 3
        self.t(nums, k)

    def test4(self):
        nums = [5, 7, 8, 8, 7, 4, 3, 6]
        k = 1
        self.t(nums, k)

if __name__ == '__main__':
    unittest.main()