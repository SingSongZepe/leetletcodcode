import unittest
from typing import List

class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        pass

class Test(unittest.TestCase):
    def test1(self):
        solution = Solution()
        intervals = [[1, 3], [6, 9]]
        newInterval = [2, 5]
        print(solution.insert(intervals, newInterval))

    def test2(self):
        solution = Solution()
        intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]]
        newInterval = [4,8]
        print(solution.insert(intervals, newInterval))