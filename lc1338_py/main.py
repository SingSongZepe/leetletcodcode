
from typing import List
from collections import defaultdict
class Solution:
    def minSetSize(self, arr: List[int]) -> int:
        mp = defaultdict(int)
        for i in arr:
            mp[i] += 1

        c = len(arr)
        t = c // 2
        freq = sorted(mp.values(), reverse=True)
        count = 0
        for fr in freq:
            if c <= t:
                break
            c -= fr
            count += 1
        return count



def main():
    print('Hello World')

if __name__ == '__main__':
    main()