import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().balancedStringSplit(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = 'RLRRLLRLRL'
        self.t(s, 4)
    

    def test2(self):
        s = 'RLRRRLLRLL'
        self.t(s, 2)
    
    def test3(self):
        s = 'LLLLRRRR'
        self.t(s, 1)


if __name__ == '__main__':
    unittest.main()