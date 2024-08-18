import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maxProfit(arg)
        print(result)
        self.assertEqual(result, expected)
    
    def t1(self, arg, expected=None):
        result = Solution1().maxProfit(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        prices = [7,1,5,3,6,4]
        expected = 5
        self.t(prices, expected)

    def test2(self):
        prices = [7,6,4,3,1]
        expected = 0
        self.t(prices, expected)

    def test11(self):
        prices = [7,1,5,3,6,4]
        expected = 5
        self.t1(prices, expected)

    def test21(self):
        prices = [7,6,4,3,1]
        expected = 0
        self.t1(prices, expected)

if __name__ == '__main__':
    unittest.main()