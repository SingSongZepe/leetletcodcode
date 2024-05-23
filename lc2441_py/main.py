
from typing import List
class Solution:
    def findMaxK(self, nums: List[int]) -> int:
        mp = {}
        m = -1
        for n in nums:
            if -n in mp:
                m = max((n if n > 0 else -n), m)
            else:
                mp[n] = True
        return m

class Solution1:
    def findMaxK(self, nums: List[int]) -> int:
        def abs(n: int) -> int:
            return n if n > 0 else -n

        mp = [False] * 1000
        m = -1
        for n in nums:
            if mp[abs(n)]:
                m = max(m, abs(n))
            else:
                mp[abs(n)] = True
        return m


def main():
    print('Hello World')

if __name__ == '__main__':
    main()