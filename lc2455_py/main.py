

from typing import List
class Solution:
    def averageValue(self, nums: List[int]) -> int:
        v = [n for n in nums if n % 6 == 0]
        return sum(v) // len(v) if v else 0


def main():
    print('Hello World')

if __name__ == '__main__':
    main()