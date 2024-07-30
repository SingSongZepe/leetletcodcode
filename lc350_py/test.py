import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, nums1, nums2, expected=None):
        result = Solution().intersect(nums1, nums2)
        print(result)

    def t1(self, nums1, nums2, expected=None):
        result = Solution1().intersect(nums1, nums2)
        print(result)
        
    def test1(self):
        nums1 = [1, 2, 2, 1]
        nums2 = [2, 2]
        self.t(nums1, nums2)

    def test2(self):
        nums1 = [4, 9, 5]
        nums2 = [9, 4, 9, 8, 4]
        self.t(nums1, nums2)

    def test11(self):
        nums1 = [1, 2, 2, 1]
        nums2 = [2, 2]
        self.t1(nums1, nums2)

    def test21(self):
        nums1 = [4, 9, 5]
        nums2 = [9, 4, 9, 8, 4]
        self.t1(nums1, nums2)

    def test_dict(self):
        nums1 = [1, 2, 2, 1]
        nums2 = [2, 2]
        d = Counter(nums1)
        print(list(d))

        d[1] = 0
        print(list(d))

if __name__ == '__main__':
    unittest.main()