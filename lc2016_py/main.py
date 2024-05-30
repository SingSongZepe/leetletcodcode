
from typing import List
class Solution:
    def maximumDifference(self, nums: List[int]) -> int:
        mi = nums[0]
        max_diff = 0

        for i in range(1, len(nums)):
            mi = min(mi, nums[i])
            max_diff = max(max_diff, nums[i] - mi)

        return max_diff if max_diff > 0 else -1

def main():
    print('Hello World')

if __name__ == '__main__':
    main()