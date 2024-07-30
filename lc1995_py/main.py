
from typing import List

from collections import defaultdict
class Solution:
    def countQuadruplets(self, nums: List[int]) -> int:
        sums = defaultdict(int)
        count = 0

        for b in range(1, len(nums)-2):
            c = b + 1
            for a in range(b):
                sums[nums[a]+nums[b]] += 1
            for d in range(c+1, len(nums)):
                count += sums[nums[d]-nums[c]]
        return count



def main():
    print('Hello World')

if __name__ == '__main__':
    main()