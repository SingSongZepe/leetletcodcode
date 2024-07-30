

from typing import List
from collections import Counter
class Solution:
    def divideArray(self, nums: List[int]) -> bool:
        return all(~v & 1 for v in Counter(nums).values())


def main():
    print('Hello World')

if __name__ == '__main__':
    main()