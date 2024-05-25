
from typing import List
class Solution:
    def countElements(self, nums: List[int]) -> int:
        min_val = 100001
        min_count = 1
        max_val = -100001
        max_count = 1

        for num in nums:
            if num == min_val:
                min_count += 1
            if num == max_val:
                max_count += 1
            if num < min_val:
                min_val = num
                min_count = 1
            if num > max_val:
                max_val = num
                max_count = 1

        return len(nums) - min_count - max_count if min_val != max_val else len(nums) - min_count

class Solution1:
    def countElements(self, nums: List[int]) -> int:
        min_val = 100001
        max_val = -100001

        for num in nums:
            if num < min_val:
                min_val = num
            if num > max_val:
                max_val = num

        return len(nums) - nums.count(min_val) - nums.count(max_val) if min_val != max_val else len(nums) - nums.count(min_val)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()