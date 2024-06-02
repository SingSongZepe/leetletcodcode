

from typing import List
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:

        l = len(nums)
        dp = [1] * l

        for j in range(1, l):
            for i in range(j):
                if nums[i] < nums[j]:
                    dp[j] = max(dp[j], dp[i] + 1)

        return max(dp)

import bisect
class Solution1:
    def lengthOfLIS(self, nums: List[int]) -> int:
        dp = []
        for n in nums:
            ind = bisect.bisect_left(dp, n)
            if ind == len(dp):
                dp.append(n)
            else:
                dp[ind] = n
        return len(dp)

class Solution2:
    def lengthOfLIS(self, nums: List[int]) -> int:
        def find(arr, x) -> int:
            def recurse(arr, x, low, high) -> int:
                if low > high:
                    return high + 1
                mid = (low + high) // 2
                if arr[mid] == x:
                    return mid
                elif arr[mid] < x:
                    return recurse(arr, x, mid + 1, high)
                else:
                    return recurse(arr, x, low, mid - 1)
            return recurse(arr, x, 0, len(arr) - 1)

        dp = []
        for n in nums:
            idx = find(dp, n)
            if idx == len(dp):
                dp.append(n)
            else:
                dp[idx] = n
        return len(dp)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()