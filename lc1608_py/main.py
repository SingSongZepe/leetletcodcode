
from typing import List
class Solution:
    def specialArray(self, nums: List[int]) -> int:
        nums.sort()
        l = len(nums)
        for i in range(1, l+1):
            if (i != l and nums[l - i] >= i > nums[l - i - 1]) or (i == l and nums[0] >= i):
                return i
        return -1

def main():
    print('Hello World')

if __name__ == '__main__':
    main()