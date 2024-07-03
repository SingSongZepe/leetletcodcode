import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, meetings, expected=None):
        result = Solution().countDays(arg, meetings)
        print(result)
        
    def test1(self):
        days = 10
        meetings = [[5,7],[1,3],[9,10]]
        expected = 2
        self.t(days, meetings, expected)

    def test2(self):
        days = 5
        meetings = [[2, 4], [1, 3]]
        expected = 1
        self.t(days, meetings, expected)

    def test3(self):
        days = 6
        meetings = [[1, 6]]
        expected = 0
        self.t(days, meetings, expected)

if __name__ == '__main__':
    unittest.main()