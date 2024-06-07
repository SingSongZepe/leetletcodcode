import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maximumValue(arg)
        print(result)
        
    def test1(self):
        strs = ["alic3","bob","3","4","00000"]
        self.t(strs)

    def test2(self):
        strs = ['1', '01', '001', '0001']
        self.t(strs)

if __name__ == '__main__':
    unittest.main()