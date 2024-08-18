import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, k, nums, l,  expected=None):
        res = []

        kl = KthLargest(k, nums)
        for i in l:
            res.append(kl.add(i))
        
        self.assertEqual(res, expected)

    def test1(self):
        k = 3
        nums = [4, 5, 8, 2]
        l = [3, 5, 10, 9, 4]
        expected = [4, 5, 5, 8, 8]
        self.t(k, nums, l, expected)

if __name__ == '__main__':
    unittest.main()