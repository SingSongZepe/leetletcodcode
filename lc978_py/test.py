import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arr, expected=None):
        result = Solution().maxTurbulenceSize(arr)
        print(result)

    def test1(self):
        arr = [9,4,2,10,7,8,8,1,9]
        self.t(arr) # Output: 5

    def test2(self):
        arr = [4,8,12,16]
        self.t(arr) # Output: 2

    def test3(self):
        arr = [100]
        self.t(arr) # Output: 1

if __name__ == '__main__':
    unittest.main()