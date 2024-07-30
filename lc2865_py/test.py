import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maximumSumOfHeights(arg)
        print(result)

    def t1(self, arg, expected=None):
        result = Solution1().maximumSumOfHeights(arg)
        print(result)

    def t2(self, arg, expected=None):
        result = Solution2().maximumSumOfHeights(arg)
        print(result)
        
    def test1(self):
        heights = [5, 3, 4, 1, 1]
        self.t(heights) # expected output: 13

    def test2(self):
        heights = [6, 5, 3, 9, 2, 7]
        self.t(heights) # expected output: 22

    def test3(self):
        heights = [3, 2, 5, 5, 2, 3]
        self.t(heights) # expected output: 18

    def test11(self):
        heights = [5, 3, 4, 1, 1]
        self.t1(heights) # expected output: 13

    def test21(self):
        heights = [6, 5, 3, 9, 2, 7]
        self.t1(heights) # expected output: 22

    def test31(self):
        heights = [3, 2, 5, 5, 2, 3]
        self.t1(heights) # expected output: 18

    def test12(self):
        heights = [5, 3, 4, 1, 1]
        self.t2(heights) # expected output: 13

    def test22(self):
        heights = [6, 5, 3, 9, 2, 7]
        self.t2(heights) # expected output: 22

    def test32(self):
        heights = [3, 2, 5, 5, 2, 3]
        self.t2(heights) # expected output: 18


    def test_non_decreasing(self):
        heights = [5, 6, 2, 4, 5, 3, 7] # [2, 2, 2, 3, 3, 3, 7]
        self.t(heights)


if __name__ == '__main__':
    unittest.main()