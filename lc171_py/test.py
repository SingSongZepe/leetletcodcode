import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, columnTitle, expected=None):
        result = Solution().titleToNumber(columnTitle)
        print(result)

    def test1(self):
        columnTitle = "A"
        self.t(columnTitle) # expected 1

    def test2(self):
        columnTitle = "AB"
        self.t(columnTitle) # expected 28

    def test3(self):
        columnTitle = "ZY"
        self.t(columnTitle) # expected 701

if __name__ == '__main__':
    unittest.main()