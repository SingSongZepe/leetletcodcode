import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().convertTemperature(arg)
        print(result)
        
    def test1(self):
        celsius = 36.5
        result = Solution().convertTemperature(celsius)

    def test2(self):
        celsius = 122.11
        result = Solution().convertTemperature(celsius)

if __name__ == '__main__':
    unittest.main()