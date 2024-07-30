
from typing import List

from collections import defaultdict
class Solution:
    def minOperations(self, nums: List[int]) -> int:
        mp = defaultdict(int)
        for n in nums:
            mp[n] += 1

        count = 0
        for n in mp:
            found = False
            for i in range(mp[n] // 2 + 1):
                rm = (mp[n] - 2 * i)
                if rm % 3 == 0:
                    count += i + rm // 3
                    found = True
                    break
            if not found:
                return -1
        return count

from collections import Counter
class Solution1:
    def minOperations(self, nums: List[int]) -> int:
        frequency_map = Counter(nums)
        count = 0

        for freq in frequency_map.values():
            if freq == 1:
                return -1
            count += freq // 3
            if freq % 3 != 0:
                count += 1
        return count

class Solutio2:
    def minOperations(self, nums: List[int]) -> int:
        mp = Counter(nums)
        count = 0

        for n in mp:
            if mp[n] == 1:
                return -1
            count += mp[n] // 3
            if mp[n] % 3 != 0:
                count += 1
        return count



def main():
    print('Hello World')

if __name__ == '__main__':
    main()