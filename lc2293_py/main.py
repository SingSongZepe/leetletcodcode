
from typing import List
class Solution:
    def minMaxGame(self, nums: List[int]) -> int:

        l = len(nums)

        while l > 1:
            for i in range(l // 2):
                if i % 2 == 0:
                    nums[i] = min(nums[2*i], nums[2*i+1])
                else:
                    nums[i] = max(nums[2*i], nums[2*i+1])
            l = l // 2

        return nums[0]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()