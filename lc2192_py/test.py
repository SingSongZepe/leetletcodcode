import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, edgeList, expected=None):
        result = Solution().getAncestors(arg, edgeList)
        print(result)
        
    def test1(self):
        # Input: n = 5, edgeList = [[0, 1], [0, 2], [0, 3], [0, 4], [1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        # Output: [[], [0], [0, 1], [0, 1, 2], [0, 1, 2, 3]]
        n = 5
        edgeList = [[0, 1], [0, 2], [0, 3], [0, 4], [1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        expected = [[], [0], [0, 1], [0, 1, 2], [0, 1, 2, 3]]
        self.t(n, edgeList, expected)

    def test2(self):
        # Input: n = 8, edgeList = [[0, 3], [0, 4], [1, 3], [2, 4], [2, 7], [3, 5], [3, 6], [3, 7], [4, 6]]
        # Output: [[], [], [], [0, 1], [0, 2], [0, 1, 3], [0, 1, 2, 3, 4], [0, 1, 2, 3]]
        n = 8
        edgeList = [[0, 3], [0, 4], [1, 3], [2, 4], [2, 7], [3, 5], [3, 6], [3, 7], [4, 6]]
        expected = [[], [], [], [0, 1], [0, 2], [0, 1, 3], [0, 1, 2, 3, 4], [0, 1, 2, 3]]
        self.t(n, edgeList, expected)

if __name__ == '__main__':
    unittest.main()