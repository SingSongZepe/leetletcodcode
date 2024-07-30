
from typing import List
from collections import Counter
class Solution:
    def findFinalValue(self, nums: List[int], original: int) -> int:
        c = Counter(nums)
        while c[original]:
            original *= 2
        return original


class Solution1:
    def findFinalValue(self, nums: List[int], original: int) -> int:
        while original in nums:
            original *= 2
        return original


def main():
    print('Hello World')

if __name__ == '__main__':
    main()