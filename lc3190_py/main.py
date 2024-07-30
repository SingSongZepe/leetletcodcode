

from typing import List

class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        return sum(1 for n in nums if n % 3 != 0)

class Solution1:
    def minimumOperations(self, nums: List[int]) -> int:
        s = 0
        for n in nums:
            if n % 3 != 0:
                s += 1
        return s

def main():
    print('Hello World')

if __name__ == '__main__':
    main()