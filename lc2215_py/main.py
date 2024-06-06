
from typing import List
class Solution:
    def findDifference(self, nums1: List[int], nums2: List[int]) -> List[List[int]]:
        s1 = set(nums1)
        s2 = set(nums2)
        return [list(s1.difference(s2)), list(s2.difference(s1))]



def main():
    print('Hello World')

if __name__ == '__main__':
    main()