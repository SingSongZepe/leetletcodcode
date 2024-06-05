import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().strongPasswordCheckerII(arg)
        print(result)
        
    def test1(self):
        password = "IloveLe3tcode!"
        self.t(password)

    def test2(self):
        password = "Me+You--IsMyDream"
        self.t(password)

    def test3(self):
        password = "1aB!"
        self.t(password)

if __name__ == '__main__':
    unittest.main()