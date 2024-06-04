import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, details, expected=None):
        result = Solution().countSeniors(details)
        print(result)

    def test1(self):
        details = ["7868190130M7522","5303914400F9211","9273338290F4010"]
        self.t(details)

    def test2(self):
        details = ["1313579440F2036", "2921522980M5644"]
        self.t(details)

    def test3(self):
        details = ["5612624052M0130","5378802576M6424","5447619845F0171","2941701174O9078"]
        self.t(details)

if __name__ == '__main__':
    unittest.main()