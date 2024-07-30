

from typing import List

class Solution:
    def evenOddBit(self, n: int) -> List[int]:
        res = [0, 0]
        while n:
            if n & 1:
                res[0] += 1
            if n & 2:
                res[1] += 1
            n >>= 2
        return res

class Solution1:
    def evenOddBit(self, n: int) -> List[int]:
        e, o = 0, 0
        while n:
            if n & 1:
                e += 1
            if n & 2:
                o += 1
            n >>= 2
        return [e, o]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()