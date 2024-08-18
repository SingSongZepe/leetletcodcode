import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maxScoreSightseeingPair(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        values = [8,1,5,2,6]
        expected = 11
        self.t(values, expected)
    
    def test2(self):
        values = [1,2]
        expected = 2
        self.t(values, expected)

if __name__ == '__main__':
    unittest.main()