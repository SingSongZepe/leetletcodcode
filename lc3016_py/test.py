import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().minimumPushes(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = 'abcde'
        expected = 5
        self.t(s, expected)
    
    def test2(self):
        s = 'xyzxyzxyzxyz'
        expected = 12
        self.t(s, expected)
    
    def test3(self):
        s = 'aabbccddeeffgghhiiiiii'
        expected = 24
        self.t(s, expected)
    

if __name__ == '__main__':
    unittest.main()