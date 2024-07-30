from typing import List
import unittest
import itertools

class Solution:
    # good memory perfomance 99.5%
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda x: x[0])
        result = [intervals[0]]
        for il in itertools.islice(intervals, 1, None):
            if il[1] <= result[-1][1]: ######### ??????
                continue
            elif il[0] <= result[-1][1]:
                result[-1][1] = il[1]
            else:
                result.append(il)
        return result

    # good speed performance 99.75%
    def merge1(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals = sorted(intervals, key=lambda x: x[0])
        ans = [intervals[0]]
        for interval in itertools.islice(intervals, 1, None):
            if ans[-1][1] < interval[0]:
                ans.append(interval)
            else:
                ans[-1][1] = max(ans[-1][1], interval[1])
        return ans

    def merge2(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda x: x[0])
        result = [intervals[0]]
        for interval in itertools.islice(intervals, 1, None):
            if result[-1][1] < interval[0]:
                result.append(interval)
            else:
                result[-1][1] = max(result[-1][1], interval[1])
        return result

class Test(unittest.TestCase):
    def test1(self):
        solution = Solution()
        intervals = [[1,3],[2,6],[8,10],[15,18]]
        result = solution.merge(intervals)
        print(result)
        # [[1,6],[8,10],[15,18]]

    def test2(self):
        solution = Solution()
        intervals = [[1,4],[4,5]]
        result = solution.merge(intervals)
        print(result)
        #  [[1,5]]

    def test3(self):
        solution = Solution()
        intervals = [[1,4],[0,2],[3,5],[7,9]]
        result = solution.merge(intervals)
        print(result)

    def test4(self):
        solution = Solution()
        intervals = [[1,4], [2,3]]
        result = solution.merge(intervals)
        print(result)

    def test11(self):
        solution = Solution()
        intervals = [[1,3],[2,6],[8,10],[15,18]]
        result = solution.merge1(intervals)
        print(result)
        # [[1,6],[8,10],[15,18]]

    def test21(self):
        solution = Solution()
        intervals = [[1,4],[4,5]]
        result = solution.merge1(intervals)
        print(result)
        #  [[1,5]]

    def test31(self):
        solution = Solution()
        intervals = [[1,4],[0,2],[3,5],[7,9]]
        result = solution.merge1(intervals)
        print(result)

    def test41(self):
        solution = Solution()
        intervals = [[1,4], [2,3]]
        result = solution.merge1(intervals)
        print(result)
