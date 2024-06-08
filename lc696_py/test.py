import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().countBinarySubstrings(arg)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, expected=None):
        result = Solution1().countBinarySubstrings(arg)
        print(result)
        self.assertEqual(result, expected)

    def t2(self, arg, expected=None):
        result = Solution2().countBinarySubstrings(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = "00110011"
        self.t(s, 6)

    def test2(self):
        s = "10101"
        self.t(s, 4)

    def test11(self):
        s = "00110011"
        self.t1(s, 6)


    def test21(self):
        s = '10101'
        self.t1(s, 4)


    def test12(self):
        s = "00110011"
        self.t2(s, 6)


    def test22(self):
        s = '10101'
        self.t2(s, 4)



if __name__ == '__main__':
    unittest.main()