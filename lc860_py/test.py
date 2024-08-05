import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().lemonadeChange(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        bills = [5,5,5,10,20]
        expected = True
        self.t(bills, expected)
    
    def test2(self):
        bills = [5,5,10,10,20]
        expected = False
        self.t(bills, expected)



if __name__ == '__main__':
    unittest.main()