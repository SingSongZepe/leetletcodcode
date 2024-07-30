import typing


class Solution:
    def pivotInteger(self, n: int) -> int:
        subsum = n * (n+1) // 2
        presum = 0
        for i in range(1, n+1):
            presum += i
            if subsum == presum:
                return i
            subsum -= i
        return -1

import math
class Solution1:
    def pivotInteger(self, n: int) -> int:
        a = math.sqrt((n**2+n)/2)
        return int(a) if int(a) == a else -1


def finder(to: int) -> [int]:
    res = []
    for i in range(1, to+1):
        if Solution1().pivotInteger(i) == -1:
            continue
        else:
            res.append(i)

    return res


def main():
    print('Hello World')

if __name__ == '__main__':
    main()