import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = '1101'
        result = Solution().numSteps(s)
        print(result)

    def test2(self):
        s = '10'
        result = Solution().numSteps(s)
        print(result)

    def test3(self):
        s = '1'
        result = Solution().numSteps(s)
        print(result)

    def test11(self):
        s = '1101'
        result = Solution1().numSteps(s)
        print(result)

    def test21(self):
        s = '10'
        result = Solution1().numSteps(s)
        print(result)

    def test31(self):
        s = '1'
        result = Solution1().numSteps(s)
        print(result)

if __name__ == '__main__':
    unittest.main()