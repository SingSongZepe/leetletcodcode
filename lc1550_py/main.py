
from typing import List

class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        count = 0
        for i in range(len(arr)):
            if arr[i] & 1:
                count += 1
                if count == 3:
                    return True
            else:
                count = 0
        return False

class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        for i in range(len(arr)-2):
            if arr[i] & 1 and arr[i+1] & 1 and arr[i+2] & 1:
                return True
        return False


def main():
    print('Hello World')

if __name__ == '__main__':
    main()