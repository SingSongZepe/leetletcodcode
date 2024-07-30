

from typing import List
class Solution:
    def findLengthOfShortestSubarray(self, arr: List[int]) -> int:

        pass


class Solution1:
    def findLengthOfShortestSubarray(self, arr: List[int]) -> int:
        # sentinel
        arr.append(float("inf"))
        arr.insert(0, 0)

        left = 0
        right = len(arr) - 1
        shortest = float("inf")

        while left < len(arr) - 2 and arr[left] <= arr[left + 1]:
            left += 1

        while left >= 0:
            while right - 1 > left and arr[right - 1] >= arr[left] and arr[right] >= arr[right - 1]:
                right -= 1
            shortest = min(shortest, right - left - 1)
            left -= 1

        return shortest


def main():
    print('Hello World')

if __name__ == '__main__':
    main()