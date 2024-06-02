import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arr, expected=None):
        result = Solution().minSetSize(arr)
        print(result)

    def test1(self):
        arr = [3, 3, 3, 3, 5, 5, 5, 2, 2, 7]
        self.t(arr) # expected: 2

    def test2(self):
        arr = [7, 7, 7, 7, 7, 7]
        self.t(arr) # expected: 1

if __name__ == '__main__':
    unittest.main()