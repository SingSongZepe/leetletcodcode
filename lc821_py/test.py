import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, c, expected=None):
        result = Solution().shortestToChar(s, c)
        print(result)
        
    def test1(self):
        s = 'loveleetcode'
        c = 'e'
        self.t(s, c)

    def test2(self):
        s = 'aaab'
        c = 'b'
        self.t(s, c)

if __name__ == '__main__':
    unittest.main()