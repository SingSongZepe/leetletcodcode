

from typing import List

class Solution:
    def minSwaps(self, nums: List[int]) -> int:
        n = nums.count(1)
        c = nums[:n].count(1)
        mc = c
        nums += nums[:n-1]

        for i in range(1, len(nums)-n+1):
            c += nums[i+n-1] - nums[i-1]
            mc = max(mc, c)

        return n - mc


# a better solution, use the feature of list negative indexing
class Solution:
    def minSwaps(self, nums: List[int]) -> int:

        k = sum(nums)
        ones = sum(nums[-k:])
        max_fill = ones
        for i in range(len(nums)):
            ones += nums[i]-nums[i-k]
            max_fill = max(max_fill, ones)
        return k - max_fill

        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()