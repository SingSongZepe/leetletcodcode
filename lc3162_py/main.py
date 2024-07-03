
from typing import List

class Solution:
    def numberOfPairs(self, nums1: List[int], nums2: List[int], k: int) -> int:
        count = 0
        for n1 in nums1:
            for n2 in nums2:
                if n1 % (n2 * k) == 0:
                    count += 1

        return count

class Solution1:
    def numberOfPairs(self, nums1: List[int], nums2: List[int], k: int) -> int:

        pass


def main():
    print('Hello World')

if __name__ == '__main__':
    main()