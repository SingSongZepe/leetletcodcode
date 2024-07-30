import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, matches, expected=None):
        result = Solution().findWinners(matches)
        print(result)

    def t1(self, matches, expected=None):
        result = Solution1().findWinners(matches)
        print(result)

    def test1(self):
        matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
        self.t(matches)

    def test2(self):
        matches = [[2, 3], [1, 3], [5, 4], [6, 4]]
        self.t(matches)

    def test11(self):
        matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
        self.t1(matches)

    def test21(self):
        matches = [[2, 3], [1, 3], [5, 4], [6, 4]]
        self.t1(matches)

if __name__ == '__main__':
    unittest.main()