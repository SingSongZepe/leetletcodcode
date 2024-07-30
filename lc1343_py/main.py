
from typing import List
class Solution:
    def numOfSubarrays(self, arr: List[int], k: int, threshold: int) -> int:
        count = 0
        curr = 0
        total = threshold * k
        for i in range(k):
            curr += arr[i]

        if curr >= total:
            count += 1
        for j in range(k, len(arr)):
            curr += arr[j] - arr[j-k]
            if curr >= total:
                count += 1
        return count


def main():
    print('Hello World')

if __name__ == '__main__':
    main()