import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, data, expected=None):
        result = Solution().dayOfYear(data)
        print(result)

    def test1(self):
        date = "2019-01-09"
        self.t(date) # Output: 9

    def test2(self):
        date = "2019-02-10"
        self.t(date) # Output: 41

if __name__ == '__main__':
    unittest.main()