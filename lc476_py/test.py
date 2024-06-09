import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().findComplement(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        num = 5
        self.t(num, 2)

    def test2(self):
        num = 1
        self.t(num, 0)

    def test3(self):
        num = 8
        self.t(num, 7)


if __name__ == '__main__':
    unittest.main()