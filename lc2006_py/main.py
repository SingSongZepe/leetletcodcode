
from typing import List
from collections import defaultdict
class Solution:
    # O(n) time O(n) space
    def countKDifference(self, nums: List[int], k: int) -> int:
        mp = defaultdict(int)
        count = 0
        for num in nums:
            count += mp[num-k] + mp[num+k]
            mp[num] += 1
        return count

class Solution1:
    # O(n**2) time O(1) space
    def countKDifference(self, nums: List[int], k: int) -> int:
        count = 0
        for i in range(len(nums)):
            for j in range(i+1, len(nums)):
                if abs(nums[i]-nums[j]) == k:
                    count += 1
        return count


def main():
    print('Hello World')

if __name__ == '__main__':
    main()