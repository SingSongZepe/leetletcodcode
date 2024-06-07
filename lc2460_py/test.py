import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().applyOperations(arg)
        print(result)
        
    def test1(self):
        nums = [1, 2, 2, 1, 1, 0]
        self.t(nums)

    def test2(self):
        nums = [0, 1]
        self.t(nums)

    def test3(self):
        nums = [847,847,0,0,0,399,416,416,879,879,206,206,206,272]
        self.t(nums)

    def test4(self):
        nums = [312,312,436,892,0,0,528,0,686,516,0,0,0,0,0,445,445,445,445,445,445,984,984,984,0,0,0,0,168,0,0,647,41,203,203,241,241,0,628,628,0,875,875,0,0,0,803,803,54,54,852,0,0,0,958,195,590,300,126,0,0,523,523]
        # nums = [0,0,523,523]
        self.t(nums)



if __name__ == '__main__':
    unittest.main()