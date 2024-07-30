import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, arr2, expected=None):
        result = Solution().relativeSortArray(arg, arr2)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        arr1 = [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19]
        arr2 = [2, 1, 4, 3, 9, 6]
        self.t(arr1, arr2, [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19])

    def test2(self):
        arr1 = [28, 6, 22, 8, 44, 17]
        arr2 = [22, 28, 8, 6]
        self.t(arr1, arr2, [22, 28, 8, 6, 17, 44])


if __name__ == '__main__':
    unittest.main()