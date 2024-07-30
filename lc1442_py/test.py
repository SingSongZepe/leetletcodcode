import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arr, expected=None):
        result = Solution().countTriplets(arr)
        print(result)

    def t1(self, arr, expected=None):
        result = Solution1().countTriplets(arr)
        print(result)

    def test1(self):
        arr = [2, 3, 1, 6, 7]
        self.t(arr)

    def test2(self):
        arr = [1, 1, 1, 1, 1]
        self.t(arr)

    def test11(self):
        arr = [2, 3, 1, 6, 7]
        self.t1(arr)

    def test21(self):
        arr = [1, 1, 1, 1, 1]
        self.t1(arr)




if __name__ == '__main__':
    unittest.main()