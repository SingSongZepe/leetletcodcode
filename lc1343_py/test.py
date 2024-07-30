import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arr, k, threshold, expected=None):
        result = Solution().numOfSubarrays(arr, k, threshold)
        print(result)

    def test1(self):
        arr = [2,2,2,2,5,5,5,8]
        k = 3
        threshold = 4
        self.t(arr, k, threshold)

    def test2(self):
        arr = [11, 13, 17, 23, 29, 31, 7, 5, 2, 3]
        k = 3
        threshold = 5
        self.t(arr, k, threshold)


if __name__ == '__main__':
    unittest.main()