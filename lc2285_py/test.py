import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, roads, expected=None):
        result = Solution().maximumImportance(arg, roads)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        n = 5
        roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
        expected = 43
        self.t(n, roads, expected)

    def test2(self):
        n = 5
        roads = [[0, 3], [2, 4], [1, 3]]
        expected = 20
        self.t(n, roads, expected)



if __name__ == '__main__':
    unittest.main()