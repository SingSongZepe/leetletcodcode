
from typing import List
from collections import defaultdict
class Solution:
    def countTriplets(self, arr: List[int]) -> int:
        count = 0
        prefix_xor = []
        x = 0

        for j in range(1, len(arr)):
            x ^= arr[j-1]
            prefix_xor.append(x)
            mp = defaultdict(int)
            for i in range(j-1):
                mp[prefix_xor[i] ^ prefix_xor[j-1]] += 1
            mp[prefix_xor[j-1]] += 1

            v = 0
            for k in range(j, len(arr)):
                v ^= arr[k]
                count += mp[v]

        return count

class Solution1:
    def countTriplets(self, arr: List[int]) -> int:
        size = len(arr)
        count = 0
        prefix = 0

        count_map = defaultdict(int)
        count_map[0] = 1
        total_map = defaultdict(int)

        for i in range(size):
            prefix ^= arr[i]

            count += count_map[prefix] * i - total_map[prefix]

            total_map[prefix] += i + 1
            count_map[prefix] += 1

        return count

 # class Solution2:
 #    def countTriplets(self, arr: List[int]) -> int:
 #        l = len(arr)
 #        count = 0
 #


def main():
    print('Hello World')

if __name__ == '__main__':
    main()