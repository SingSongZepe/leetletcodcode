import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [1, 3, 4, 1, 2, 3, 1]
        result = Solution().findMatrix(nums)
        print(result)

    def test2(self):
        nums = [1, 2, 3, 4]
        result = Solution().findMatrix(nums)
        print(result)

    def test11(self):
        nums = [1, 3, 4, 1, 2, 3, 1]
        result = Solution1().findMatrix(nums)
        print(result)

    def test21(self):
        nums = [1, 2, 3, 4]
        result = Solution1().findMatrix(nums)
        print(result)

if __name__ == '__main__':
    unittest.main()