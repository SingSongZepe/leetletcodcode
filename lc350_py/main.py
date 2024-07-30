

from typing import List
from collections import Counter

class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        d1 = Counter(nums1)
        d2 = Counter(nums2)

        for k, v in d1.items():
            if k in d2:
                d1[k] = min(v, d2[k])
            else:
                d1[k] = 0

        return list(d1.elements())


class Solution1:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        nums1.sort()
        nums2.sort()

        res = []
        l1, l2 = len(nums1), len(nums2)
        p, q = 0, 0
        while p < l1 and q < l2:
            if nums1[p] == nums2[q]:
               res.append(nums1[p])
               p += 1
               q += 1
            elif nums1[p] < nums2[q]:
                p += 1
            else:
                q += 1
        return res


def main():
    print('Hello World')

if __name__ == '__main__':
    main()