import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().maxPoints(arg)
        print(result)
        self.assertEqual(result, expected)
    
    def t1(self, arg, expected=None):
        result = Solution1().maxPoints(arg)
        print(result)
        self.assertEqual(result, expected)
    
            
    # def test1(self):
    #     points = [[1,2,3],[1,5,1],[3,1,1]]
    #     expected = 9
    #     self.t(points, expected)
    
    # def test2(self):
    #     points = [[1,5],[3,2],[4,2]]
    #     expected = 11
    #     self.t(points, expected)
        
    def test11(self):
        points = [[1,2,3],[1,5,1],[3,1,1]]
        expected = 9
        self.t1(points, expected)
    
    def test21(self):
        points = [[1,5],[3,2],[4,2]]
        expected = 11
        self.t1(points, expected)

if __name__ == '__main__':
    unittest.main()