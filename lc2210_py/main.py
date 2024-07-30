

from typing import List
class Solution:
    def countHillValley(self, nums: List[int]) -> int:
        l = len(nums)
        prev = nums[0]
        count = 0
        for p in range(1, l-1):
            if nums[p] == nums[p+1]:
                continue
            last = p + 1

            if (prev > nums[p] and nums[last] > nums[p]) or (prev < nums[p] and nums[last] < nums[p]):
                count += 1

            prev = nums[p]

        return count


class Solution1:
    def countHillValley(self, nums: List[int]) -> int:
        ans = 0
        if nums[0] > nums[1]:
            state = 0
        elif nums[1] > nums[0]:
            state = 1
        else:
            state = -1

        for i in range(1, len(nums)):
            if nums[i - 1] == nums[i]:
                continue
            elif nums[i] > nums[i - 1]:
                if state == 0:
                    ans += 1
                    state = 1
                else:
                    state = 1
            elif nums[i] < nums[i - 1]:
                if state == 1:
                    ans += 1
                    state = 0
                else:
                    state = 0
        return ans

def main():
    print('Hello World')

if __name__ == '__main__':
    main()