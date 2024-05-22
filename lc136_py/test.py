import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [2, 2, 1]
        result = Solution().singleNumber(nums)
        print(result)

    def test2(self):
        nums = [4, 1, 2, 1, 2]
        result = Solution().singleNumber(nums)
        print(result)

    def test3(self):
        nums = [1]
        result = Solution().singleNumber(nums)
        print(result)

    def test11(self):
        nums = [2, 2, 1]
        result = Solution1().singleNumber(nums)
        print(result)

    def test21(self):
        nums = [1, 2, 1, 2, 4]
        result = Solution1().singleNumber(nums)
        print(result)

    def test31(self):
        nums = [1]
        result = Solution1().singleNumber(nums)
        print(result)


    def test12(self):
        nums = [2, 2, 1]
        result = Solution2().singleNumber(nums)
        print(result)

    def test22(self):
        nums = [1, 2, 1, 2, 4]
        result = Solution2().singleNumber(nums)
        print(result)

    def test32(self):
        nums = [1]
        result = Solution2().singleNumber(nums)
        print(result)


    def test13(self):
        nums = [2, 2, 1]
        result = Solution3().singleNumber(nums)
        print(result)

    def test23(self):
        nums = [1, 2, 1, 2, 4]
        result = Solution3().singleNumber(nums)
        print(result)

    def test33(self):
        nums = [1]
        result = Solution3().singleNumber(nums)
        print(result)

if __name__ == '__main__':
    unittest.main()