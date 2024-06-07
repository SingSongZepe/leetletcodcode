import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().numberOfCuts(arg)
        print(result)
        
    def test1(self):
        n = 4
        self.t(n)

    def test2(self):
        n = 5
        self.t(n)

    def test3(self):
        n = 1
        self.t(n)

if __name__ == '__main__':
    unittest.main()