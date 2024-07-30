import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, bank, expected=None):
        result = Solution().numberOfBeams(bank)
        print(result)

    def test1(self):
        bank = ["011001","000000","010100","001000"]
        self.t(bank)

    def test2(self):
        bank = ["000","111","000"]
        self.t(bank)

if __name__ == '__main__':
    unittest.main()