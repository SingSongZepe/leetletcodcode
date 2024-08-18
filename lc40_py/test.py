import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, candidates, target, expected=None):
        result = Solution().combinationSum2(candidates, target)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        candidates = [10,1,2,7,6,1,5]
        target = 8
        expected = [
            [1,1,6],
            [1,2,5],
            [1,7],
            [2,6]
        ]
        self.t(candidates, target, expected)

    def test2(self):
        candidates = [2,5,2,1,2]
        target = 5
        expected = [
            [1,2,2],
            [5]
        ]
        self.t(candidates, target, expected)


if __name__ == '__main__':
    unittest.main()