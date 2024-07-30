import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().heightChecker(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        heights = [1, 1, 4, 2, 1, 3]
        self.t(heights, 3)

    def test2(self):
        heights = [5, 1, 2, 3, 4]
        self.t(heights, 5)

    def test3(self):
        heights = [1, 2, 3, 4, 5]
        self.t(heights, 0)

if __name__ == '__main__':
    unittest.main()