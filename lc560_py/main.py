
from typing import List


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        count = 0
        prefix_sum = 0
        mp = {0: 1}
        for num in nums:
            prefix_sum += num
            if prefix_sum - k in mp:
                count += mp[prefix_sum - k]
            if prefix_sum not in mp:
                mp[prefix_sum] = 1
            else:
                mp[prefix_sum] += 1
        return count



def main():
    print('Hello World')

if __name__ == '__main__':
    main()