
from typing import List
class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        mp = {0: -1}
        prefix = 0
        for idx, n in enumerate(nums):
            prefix += n
            prefix %= k
            if prefix not in mp:
                mp[prefix] = idx
            else:
                if idx - mp[prefix] > 1:
                    return True
        return False


class Solution1:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        remainder_cache = {0: -1}
        remainder = 0
        for i in range(len(nums)):
            remainder += nums[i]
            remainder %= k
            if remainder not in remainder_cache:
                remainder_cache[remainder] = i
            elif i - remainder_cache[remainder] >= 2:
                return True
        return False


def main():
    print('Hello World')

if __name__ == '__main__':
    main()