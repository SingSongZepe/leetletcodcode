
from typing import List
class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:

        zeros = 0 if flowerbed[0] == 1 else 1
        can_place = 0
        for v in flowerbed:
            if v == 0:
                zeros += 1
            else:
                if zeros == 0:
                    continue
                can_place += (zeros - 1) // 2
                zeros = 0
        zeros += 1 if flowerbed[-1] == 0 else 0
        can_place += (zeros - 1) // 2 if zeros > 0 else 0
        return can_place >= n


def main():
    print('Hello World')

if __name__ == '__main__':
    main()