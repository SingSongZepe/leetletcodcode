import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, rec1, rec2, expected=None):
        result = Solution().isRectangleOverlap(rec1, rec2)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        rec1 = [0,0,2,2]
        rec2 = [1,1,3,3]
        self.t(rec1, rec2, expected=True)

    def test2(self):
        rec1 = [0,0,1,1]
        rec2 = [1,0,2,1]
        self.t(rec1, rec2, expected=False)

    def test3(self):
        rec1 = [0,0,1,1]
        rec2 = [2,2,3,3]
        self.t(rec1, rec2, expected=False)

    def test4(self):
        rec1 = [-4,-9,-2,3]
        rec2 = [1,-5,9,-1]
        self.t(rec1, rec2, expected=False)

if __name__ == '__main__':
    unittest.main()