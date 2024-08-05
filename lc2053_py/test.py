import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arr, k, expected=None):
        result = Solution().kthDistinct(arr, k)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        arr = ['d', 'b', 'c', 'b', 'c', 'a']
        k = 2
        expected = 'a'
        self.t(arr, k, expected)

    def test2(self):
        arr = ["aaa","aa","a"]
        k = 1
        expected = "aaa"
        self.t(arr, k, expected)
    
    def test3(self):
        arr = ["a","b","a"]
        k = 3
        expected = ""
        self.t(arr, k, expected)

if __name__ == '__main__':
    unittest.main()