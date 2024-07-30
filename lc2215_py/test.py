import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, nums2, expected=None):
        result = Solution().findDifference(arg, nums2)
        print(result)
        
    def test1(self):
        nums1 = [1, 2, 3]
        nums2 = [2, 4, 6]
        self.t(nums1, nums2)

    def test2(self):
        nums1 = [1, 2, 3, 3]
        nums2 = [1, 1, 2, 2]
        self.t(nums1, nums2)


if __name__ == '__main__':
    unittest.main()