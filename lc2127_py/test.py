import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, favorite, expected=None):
        result = Solution().maximumInvitations(favorite)
        print(result)

    def t1(self, favorite, expected=None):
        result = Solution1().maximumInvitations(favorite)
        print(result)

    def test1(self):
        favorite = [2,2,1,2]
        self.t(favorite)

    def test2(self):
        favorite = [1,2,0]
        self.t(favorite)

    def test3(self):
        favorite = [1,1,1,1,1]
        self.t(favorite)

    def test4(self):
        favorite = [1,2,3,4,5,6,7,8,9,0]
        self.t(favorite)

    def test5(self):
        favorite = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,0]
        self.t(favorite)

    def test11(self):
        favorite = [2,2,1,2]
        self.t1(favorite)

    def test21(self):
        favorite = [1,2,0]
        self.t1(favorite)

    def test31(self):
        favorite = [1,0,1,1,1]
        self.t1(favorite)

    def test41(self):
        favorite = [1,2,3,4,5,6,7,8,9,0]
        self.t1(favorite)

    def test51(self):
        favorite = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,0]
        self.t1(favorite)

if __name__ == '__main__':
    unittest.main()