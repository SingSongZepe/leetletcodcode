import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, target, arr, expected=None):
        result = Solution().canBeEqual(target, arr)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        target = [1, 2, 3, 4]
        arr = [2, 4, 1, 3]
        expected = True
        self.t(target, arr, expected)
    
    def test2(self):
        target = [7]
        arr = [7]
        expected = True
        self.t(target, arr, expected)

    
    def test3(self):
        target = [3, 7, 9]
        arr = [3, 7, 11]
        expected = False
        self.t(target, arr, expected)

if __name__ == '__main__':
    unittest.main()