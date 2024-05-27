import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        nums = [3, 5]
        result = Solution().specialArray(nums)
        print(result)

    def test2(self):
        nums = [0, 0]
        result = Solution().specialArray(nums)
        print(result)

    def test3(self):
        nums = [0, 4, 3, 0, 4]
        result = Solution().specialArray(nums)
        print(result)

if __name__ == '__main__':
    unittest.main()