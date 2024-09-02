import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().transpose(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        expected = [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        self.t(matrix, expected)

    def test2(self):
        matrix = [[1, 2, 3], [4, 5, 6]]
        expected = [[1, 4], [2, 5], [3, 6]]
        self.t(matrix, expected)

if __name__ == '__main__':
    unittest.main()