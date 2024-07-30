
from typing import List
class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        m, c = 0, 0
        for n in nums:
            if n == 1:
                c += 1
            else:
                if c > m:
                    m = c
                c = 0
        return max(m, c)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()