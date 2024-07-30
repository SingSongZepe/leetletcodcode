import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, words, expected=None):
        result = Solution().commonChars(words)
        print(result)

    def test1(self):
        words = ["bella","label","roller"]
        self.t(words)

    def test2(self):
        words = ["cool","lock","cook"]
        self.t(words)


if __name__ == '__main__':
    unittest.main()