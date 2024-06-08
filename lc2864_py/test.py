import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maximumOddBinaryNumber(arg)
        print(result)
        
    def test1(self):
        s = '010'
        self.t(s)

    def test2(self):
        s = '0101'
        self.t(s)



if __name__ == '__main__':
    unittest.main()
