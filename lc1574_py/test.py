import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        arr = [1,2,3,10,4,2,3,5]
        result = Solution().findLengthOfShortestSubarray(arr)
        print(result)

    def test2(self):
        arr = [5, 4, 3, 2, 1]
        result = Solution().findLengthOfShortestSubarray(arr)
        print(result)

    def test3(self):
        arr = [1, 2, 3]
        result = Solution().findLengthOfShortestSubarray(arr)
        print(result)


    def test11(self):
        arr = [1,2,3,10,4,2,3,5]
        result = Solution1().findLengthOfShortestSubarray(arr)
        print(result)

    def test21(self):
        arr = [5, 4, 3, 2, 1]
        result = Solution1().findLengthOfShortestSubarray(arr)
        print(result)

    def test31(self):
        arr = [1, 2, 3]
        result = Solution1().findLengthOfShortestSubarray(arr)
        print(result)

if __name__ == '__main__':
    unittest.main()