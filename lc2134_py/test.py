import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().minSwaps(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        nums = [0, 1, 0, 1, 1, 0, 0]
        expected = 1
        self.t(nums, expected)

    def test2(self):
        nums = [0,1,1,1,0,0,1,1,0]
        expected = 2
        self.t(nums, expected)

    def test3(self):
        nums = [1, 1, 0, 0, 1]
        expected = 0
        self.t(nums, expected)





if __name__ == '__main__':
    unittest.main()