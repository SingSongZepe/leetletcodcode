from typing import  List
import unittest


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        curr = 0
        l = len(nums)
        if l == 1:
            return True
        while True:
            if curr + nums[curr] >= l - 1:
                return True
            im = curr
            zero = 0
            for i in range(curr + 1, curr + nums[curr] + 1):
                if nums[i] == 0:
                    zero += 1
                if nums[i] + i >= nums[im] + im:
                    im = i
            if zero == nums[curr]:
                return False
            curr = im

    def canJump1(self, nums: List[int]):
        max_reachable = 0
        for i in range(len(nums)):
            if i > max_reachable:
                return False
            max_reachable = max(max_reachable, i + nums[i])
            if max_reachable >= len(nums) - 1:
                return True
        return False

    # def canJump2(self, nums: List[int]):
    #     max_reachable = 0
    #     for i in range(len(nums)):
    #         if i > max_reachable:
    #             return False
    #         if i + nums[i] > max_reachable:
    #             max_reachable = i + nums[i]
    #         if max_reachable >= len(nums) - 1:
    #             return True
    #     return False

class Test(unittest.TestCase):
    def test1(self):
        solution = Solution()
        nums = [2, 3, 1, 1, 4]
        print(solution.canJump(nums))

    def test2(self):
        solution = Solution()
        nums = [3, 2, 1, 0, 4]
        print(solution.canJump(nums))

    def test3(self):
        solution = Solution()
        nums = [0]
        print(solution.canJump(nums))

    def test4(self):
        solution = Solution()
        nums = [1, 2]
        print(solution.canJump(nums))

    def test5(self):
        solution = Solution()
        nums = [2, 0]
        print(solution.canJump(nums))


    def test11(self):
        solution = Solution()
        nums = [2, 3, 1, 1, 4]
        print(solution.canJump1(nums))

    def test21(self):
        solution = Solution()
        nums = [3, 2, 1, 0, 4]
        print(solution.canJump1(nums))

    def test31(self):
        solution = Solution()
        nums = [0]
        print(solution.canJump1(nums))

    def test41(self):
        solution = Solution()
        nums = [1, 2]
        print(solution.canJump1(nums))

    def test51(self):
        solution = Solution()
        nums = [2, 0]
        print(solution.canJump1(nums))