import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().findErrorNums(arg)
        print(result)
        
    def test1(self):
        nums = [1, 2, 2, 4]
        self.t(nums)

    def test2(self):
        nums = [1, 1]
        self.t(nums)

if __name__ == '__main__':
    unittest.main()