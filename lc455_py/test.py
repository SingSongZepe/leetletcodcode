import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        g = [1, 2, 3]
        s = [1, 1]
        result = Solution().findContentChildren(g, s)
        print(result)

    def test2(self):
        g = [1, 2]
        s = [1, 2, 3]
        result = Solution().findContentChildren(g, s)
        print(result)


if __name__ == '__main__':
    unittest.main()