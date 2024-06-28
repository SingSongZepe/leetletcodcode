import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().minimumAverage(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().minimumAverage(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        nums = [1, 9, 8, 3, 10, 5]
        expected = 5.5
        self.t(nums, expected)

    def test2(self):
        nums = [1, 2, 3, 7, 8, 9]
        expected = 5.0
        self.t(nums, expected)

    def test11(self):
        nums = [1, 9, 8, 3, 10, 5]
        expected = 5.5
        self.t1(nums, expected)

    def test21(self):
        nums = [1, 2, 3, 7, 8, 9]
        expected = 5.0
        self.t1(nums, expected)

if __name__ == '__main__':
    unittest.main()