import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, s, goal, expected=None):
        result = Solution().buddyStrings(s, goal)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        s = 'ab'
        goal = 'ba'
        expected = True
        self.t(s, goal, expected)

    def test2(self):
        s = 'ab'
        goal = 'ab'
        expected = False
        self.t(s, goal, expected)

    def test3(self):
        s = 'aa'
        goal = 'aa'
        expected = True
        self.t(s, goal, expected)
    
    def test4(self):
        s = 'aaaaaaabc'
        goal = 'aaaaaaacb'
        expected = True
        self.t(s, goal, expected)
    
    def test5(self):
        s = 'abab'
        goal = 'abab'
        expected = True
        self.t(s, goal, expected)
    
    def test6(self):
        s = 'abcd'
        goal = 'badc'
        expected = False
        self.t(s, goal, expected)

if __name__ == '__main__':
    unittest.main()