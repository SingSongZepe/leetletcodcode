import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = 'abcd'
        t = 'bcdf'
        maxCost = 3
        result = Solution().equalSubstring(s, t, maxCost)
        print(result)

    def test2(self):
        s = 'abcd'
        t = 'cdef'
        maxCost = 3
        result = Solution().equalSubstring(s, t, maxCost)
        print(result)

    def test3(self):
        s = 'abcd'
        t = 'acde'
        maxCost = 0
        result = Solution().equalSubstring(s, t, maxCost)
        print(result)

    def test11(self):
        s = 'abcd'
        t = 'bcdf'
        maxCost = 3
        result = Solution1().equalSubstring(s, t, maxCost)
        print(result)

    def test21(self):
        s = 'abcd'
        t = 'cdef'
        maxCost = 3
        result = Solution1().equalSubstring(s, t, maxCost)
        print(result)

    def test31(self):
        s = 'abcd'
        t = 'acde'
        maxCost = 0
        result = Solution1().equalSubstring(s, t, maxCost)
        print(result)


if __name__ == '__main__':
    unittest.main()