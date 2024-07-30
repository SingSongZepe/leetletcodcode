

from typing import List

from collections import Counter
class Solution:
    def unequalTriplets(self, nums: List[int]) -> int:
        counter = Counter(nums)
        cnt = 0
        lcnt = 0
        rcnt = len(nums)
        for v in counter.values():
            rcnt -= v
            cnt += lcnt*rcnt*v
            lcnt += v
        return cnt




class Solution1:
    def unequalTriplets(self, nums: List[int]) -> int:
        c = Counter(nums)
        res = 0

        left = 0
        right = len(nums)

        for _, freq in c.items():
            right -= freq
            res += left*freq*right
            left += freq
        return res

def main():
    print('Hello World')

if __name__ == '__main__':
    main()