
from typing import List

class Solution:
    def minimumAverage(self, nums: List[int]) -> float:
        nums.sort()
        mi = 50
        for i in range(len(nums)//2):
            mi = min(mi, (nums[i]+nums[len(nums)-i-1])/2)
        return mi

class Solution1:
    def minimumAverage(self, nums: List[int]) -> float:
        nums.sort()
        mi = 50
        while nums:
            mi = min(mi, (nums.pop(0) + nums.pop()) / 2)
        return mi


def main():
    print('Hello World')

if __name__ == '__main__':
    main()