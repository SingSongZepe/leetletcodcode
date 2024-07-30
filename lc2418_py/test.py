import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, names, heights, expected=None):
        result = Solution().sortPeople(names, heights)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        names = ["Mary","John","Emma"]
        heights = [180,165,170]
        expected = ["Mary","Emma","John"]

        self.t(names, heights, expected)

    def test2(self):
        names = ["Alice","Bob","Bob"]
        heights = [155,185,150]
        expected = ["Bob","Alice","Bob"]

        self.t(names, heights, expected)

if __name__ == '__main__':
    unittest.main()