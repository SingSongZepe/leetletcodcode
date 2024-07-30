import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().flipAndInvertImage(arg)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        image = [[1,1,0],[1,0,1],[0,0,0]]
        expected = [[1,0,0],[0,1,0],[1,1,1]]
        self.t(image, expected)

    def test2(self):
        image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
        expected = [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
        self.t(image, expected)

if __name__ == '__main__':
    unittest.main()