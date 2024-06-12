import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().func(arg)
        print(result)
        
    def test1(self):
        pass

if __name__ == '__main__':
    unittest.main()