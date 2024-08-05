import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().groupAnagrams(arg)
        print(result)
        
    def test1(self):
        strs = ["eat","tea","tan","ate","nat","bat"]
        expected = [["bat"],["nat","tan"],["ate","eat","tea"]]
        self.t(strs, expected)
    
    def test2(self):
        strs = ["a"]
        expected = [["a"]]
        self.t(strs, expected)
    
    def test3(self):
        strs = [""]
        expected = [[""]]
        self.t(strs, expected)


if __name__ == '__main__':
    unittest.main()