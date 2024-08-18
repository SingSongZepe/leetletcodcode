import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maxDistance(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        arrays = [[1,2,3], [4,5], [1,2,3]]
        expected = 4
        self.t(arrays, expected)

    def test2(self):
        arrays = [[1], [1]]
        expected = 0
        self.t(arrays, expected)

    def test3(self):
        arrays = [[1,4],[0,5]]
        expected = 4
        self.t(arrays, expected)

if __name__ == '__main__':
    unittest.main()