import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().threeConsecutiveOdds(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        arr = [2, 6, 4, 1]
        self.t(arr, False)

    def test2(self):
        arr = [1, 2, 34, 3, 4, 5, 7, 23, 12]
        self.t(arr, True)

    def test3(self):
        arr = [1, 1, 1]
        self.t(arr, True)


if __name__ == '__main__':
    unittest.main()