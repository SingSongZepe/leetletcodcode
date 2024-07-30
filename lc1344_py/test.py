import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, hour, minutes, expected=None):
        result = Solution().angleClock(hour, minutes)
        print(result)

    def test1(self):
        hour = 12
        minutes = 30
        self.t(hour, minutes)

    def test2(self):
        hour = 3
        minutes = 30
        self.t(hour, minutes)

if __name__ == '__main__':
    unittest.main()