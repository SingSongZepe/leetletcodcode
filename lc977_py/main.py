

from typing import List
class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        p1 = 0
        p2 = len(nums) - 1

        result = []
        while p1 <= p2:
            if abs(nums[p1]) > abs(nums[p2]):
                result.append(nums[p1] * nums[p1])
                p1 += 1
            else:
                result.append(nums[p2] * nums[p2])
                p2 -= 1

        result.reverse()
        return result



def main():
    print('Hello World')

if __name__ == '__main__':
    main()