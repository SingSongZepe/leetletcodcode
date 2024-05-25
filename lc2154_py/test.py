import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [5,3,6,1,12]
        original = 3
        result = Solution().findFinalValue(nums, original)
        print(result)

    def test2(self):
        nums = [2, 7, 9]
        original = 4
        result = Solution().findFinalValue(nums, original)
        print(result)

    def test11(self):
        nums = [5,3,6,1,12]
        original = 3
        result = Solution1().findFinalValue(nums, original)
        print(result)

    def test21(self):
        nums = [2, 7, 9]
        original = 4
        result = Solution1().findFinalValue(nums, original)
        print(result)



if __name__ == '__main__':
    unittest.main()