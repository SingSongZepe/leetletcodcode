import unittest
from typing import List

class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        if len(intervals) == 0:
            return [newInterval]
        if newInterval[1] < intervals[0][0]:
            intervals.insert(0, newInterval)
        l = len(intervals)
        li = 0 if newInterval[1] <= intervals[0][0] else -1
        ri = l-1 if newInterval[0] >= intervals[-1][1] else -1
        for (idx, il) in enumerate(intervals):
            # not cross
            if (li == -1 and ri == -1) and (newInterval[0] > il[1] and (idx == l-1 or (idx < l-1 and newInterval[1] < intervals[idx+1][1]))):
                intervals.insert(idx+1, newInterval)
                return intervals
            # contain in newInterval
            if (li == -1 and ri == -1) and newInterval[0] <= il[0] and newInterval[1] >= il[1]:
                intervals[idx] = newInterval
                return intervals
            # contain in il
            if (li == -1 and ri == -1) and newInterval[0] > il[0] and newInterval[1] < il[1]:
                return intervals
            if li == -1 and il[0] <= newInterval[0] <= il[1]: # left cross
                li = idx
            if ri == -1 and il[0] <= newInterval[1] <= il[1]:
                ri = idx
            if il[0] > newInterval[1]:
                break
        if li != -1 and ri == -1: # only left cross
            tmp = intervals[li]
            intervals[li] = [tmp[0], newInterval[1]]
            return intervals
        if li == -1 and ri != -1: # only right cross
            tmp = intervals[ri]
            intervals[ri] = [newInterval[0], tmp[1]]
            return intervals
        # left cross and right cross
        tmpl = intervals[li]
        tmpr = intervals[ri]
        intervals[li:ri+1] = [[min(tmpl[0], newInterval[0]), max(tmpr[1], newInterval[1])]]
        return intervals

    def insert1(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        l = len(intervals)
        n = 0
        while n < l and newInterval[0] > intervals[n][1]:
            n += 1
        li = n
        while n < l and newInterval[1] >= intervals[n][0]:
            newInterval[0] = min(newInterval[0], intervals[n][0])
            newInterval[1] = max(newInterval[1], intervals[n][1])
            n += 1
        intervals[li:n] = [newInterval]
        return intervals

    def insert2(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        i = len(intervals)
        result = []
        n = 0

        while n < i and intervals[n][1] < newInterval[0]:
            result.append(intervals[n])
            n += 1

        while n < i and intervals[n][0] <= newInterval[1]:
            newInterval[0] = min(newInterval[0], intervals[n][0])
            newInterval[1] = max(newInterval[1], intervals[n][1])
            n += 1

        result.append(newInterval)

        while n < i:
            result.append(intervals[n])
            n += 1

        return result

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

    def test3(self):
        solution = Solution()
        intervals = []
        newInterval = [4,8]
        print(solution.insert(intervals, newInterval))

    def test4(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [0, 0]
        print(solution.insert(intervals, newInterval))

    def test5(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [0, 6]
        print(solution.insert(intervals, newInterval))

    def test6(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [2, 3]
        print(solution.insert(intervals, newInterval))

    # insert 1
    def test11(self):
        solution = Solution()
        intervals = [[1, 3], [6, 9]]
        newInterval = [2, 5]
        print(solution.insert1(intervals, newInterval))

    def test21(self):
        solution = Solution()
        intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]]
        newInterval = [4,8]
        print(solution.insert1(intervals, newInterval))

    def test31(self):
        solution = Solution()
        intervals = []
        newInterval = [4,8]
        print(solution.insert1(intervals, newInterval))

    def test41(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [0, 0]
        print(solution.insert1(intervals, newInterval))

    def test51(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [0, 6]
        print(solution.insert1(intervals, newInterval))

    def test61(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [2, 3]
        print(solution.insert1(intervals, newInterval))

    def test7(self):
        solution = Solution()
        intervals = [[1, 5], [6, 8]]
        newInterval = [0, 9]
        print(solution.insert(intervals, newInterval))

    def test8(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [6, 8]
        print(solution.insert(intervals, newInterval))

    def test71(self):
        solution = Solution()
        intervals = [[1, 5], [6, 8]]
        newInterval = [0, 9]
        print(solution.insert1(intervals, newInterval))

    def test81(self):
        solution = Solution()
        intervals = [[1, 5]]
        newInterval = [6, 8]
        print(solution.insert1(intervals, newInterval))