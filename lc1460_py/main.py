

from typing import List

from collections import Counter

class Solution:
    def canBeEqual(self, target: List[int], arr: List[int]) -> bool:
        return Counter(target) == Counter(arr)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()