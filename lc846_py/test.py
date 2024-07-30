import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, groupSize, expected=None):
        result = Solution().isNStraightHand(arg, groupSize)
        print(result)

    def t1(self, arg, groupSize, expected=None):
        result = Solution1().isNStraightHand(arg, groupSize)
        print(result)
        
    def test1(self):
        hand = [1,2,3,6,2,3,4,7,8]
        groupSize = 3
        self.t(hand, groupSize)

    def test2(self):
        hand = [1, 2, 3, 4, 5]
        groupSize = 4
        self.t(hand, groupSize)

    def test3(self):
        hand = [2, 1]
        groupSize = 2
        self.t(hand, groupSize)

    def test11(self):
        hand = [1,2,3,6,2,3,4,7,8]
        groupSize = 3
        self.t1(hand, groupSize)

    def test21(self):
        hand = [1, 2, 3, 4, 5]
        groupSize = 4
        self.t1(hand, groupSize)

    def test31(self):
        hand = [2, 1]
        groupSize = 2
        self.t1(hand, groupSize)

if __name__ == '__main__':
    unittest.main()