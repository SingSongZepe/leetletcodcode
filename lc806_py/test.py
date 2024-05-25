import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        widths = [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10]
        s = "abcdefghijklmnopqrstuvwxyz"
        result = Solution().numberOfLines(widths, s)
        print(result)

    def test2(self):
        widths = [4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
        s = "bbbcccdddaaa"
        result = Solution().numberOfLines(widths, s)
        print(result)

if __name__ == '__main__':
    unittest.main()