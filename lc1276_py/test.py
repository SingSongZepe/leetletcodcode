import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        tomatoSlices = 16
        chesseSlices = 7
        result = Solution().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test2(self):
        tomatoSlices = 17
        chesseSlices = 4
        result = Solution().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test3(self):
        tomatoSlices = 4
        chesseSlices = 17
        result = Solution().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test4(self):
        tomatoSlices = 0
        chesseSlices = 0
        result = Solution().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test5(self):
        tomatoSlices = 2385088
        chesseSlices = 164763
        result = Solution().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test11(self):
        tomatoSlices = 16
        chesseSlices = 7
        result = Solution1().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test21(self):
        tomatoSlices = 17
        chesseSlices = 4
        result = Solution1().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test31(self):
        tomatoSlices = 4
        chesseSlices = 17
        result = Solution1().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test41(self):
        tomatoSlices = 0
        chesseSlices = 0
        result = Solution1().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

    def test51(self):
        tomatoSlices = 2385088
        chesseSlices = 164763
        result = Solution1().numOfBurgers(tomatoSlices, chesseSlices)
        print(result)

if __name__ == '__main__':
    unittest.main()