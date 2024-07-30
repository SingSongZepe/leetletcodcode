import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, nums2, k, expected=None):
        result = Solution().numberOfPairs(arg, nums2, k)
        print(result)
        
    def test1(self):
        nums1 = [1, 3, 4]
        nums2 = [1, 3, 4]
        k = 1
        self.t(nums1, nums2, k, 5)

    def test2(self):
        nums1 = [1, 2, 4, 12]
        nums2 = [2, 4]
        k = 3
        self.t(nums1, nums2, k, 2)


if __name__ == '__main__':
    unittest.main()