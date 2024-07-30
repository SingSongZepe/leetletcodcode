import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().minimumOperations(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        nums = [1, 2, 3, 4]
        self.t(nums, 3)

    def test2(self):
        nums = [3, 6, 9]
        self.t(nums, 0)

if __name__ == '__main__':
    unittest.main()