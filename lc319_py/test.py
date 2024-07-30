import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().bulbSwitch(arg)
        print(result)
        
    def test1(self):
        n = 3
        self.t(n, 1)

    def test2(self):
        n = 0
        self.t(n, 0)

    def test3(self):
        n = 1
        self.t(n, 1)

if __name__ == '__main__':
    unittest.main()