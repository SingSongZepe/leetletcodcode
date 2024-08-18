import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s1, s2, expected=None):
        result = Solution().uncommonFromSentences(s1, s2)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s1 = "this apple is sweet"
        s2 = "this apple is sour"
        expected = ["sweet", "sour"]
        self.t(s1, s2, expected)

    def test2(self):
        s1 = "apple apple"
        s2 = "banana"
        expected = ["banana"]
        self.t(s1, s2, expected)

        
if __name__ == '__main__':
    unittest.main()