
from typing import List
class Solution:
    def distinctAverages(self, nums: List[int]) -> int:
        nums.sort()
        return len(set([(nums[i] + nums[len(nums) - i - 1]) / 2 for i in range(len(nums) // 2)]))


def main():
    print('Hello World')

if __name__ == '__main__':
    main()