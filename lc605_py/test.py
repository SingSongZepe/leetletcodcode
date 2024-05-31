import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, flowerbed, n, expected=None):
        result = Solution().canPlaceFlowers(flowerbed, n)
        print(result)

    def test1(self):
        flowerbed = [1,0,0,0,1]
        n = 1
        self.t(flowerbed, n)

    def test2(self):
        flowerbed = [1,0,0,0,1]
        n = 2
        self.t(flowerbed, n)

    def test3(self):
        nums = [0,0,1,0,1]
        n = 1
        self.t(nums, n)

if __name__ == '__main__':
    unittest.main()