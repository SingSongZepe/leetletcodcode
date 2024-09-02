import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, chalk, k, expected=None):
        result = Solution().chalkReplacer(chalk, k)
        print(result)
        
    def test1(self):
        chalk = [5, 1, 5]
        k = 22
        expected = 0
        self.t(chalk, k, expected)

    def test2(self):
        chalk = [3, 4, 1, 2]
        k = 25
        expected = 1
        self.t(chalk, k, expected)

if __name__ == '__main__':
    unittest.main()