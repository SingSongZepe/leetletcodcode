

from typing import List
from collections import Counter
class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        c = Counter(nums)
        repeat = -1
        missing = -1
        for i in range(1, len(nums)+1):
            if i not in c:
                missing = i
            elif c[i] != 1:
                repeat = i
        return [repeat, missing]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()