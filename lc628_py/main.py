
from typing import List
class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        n = len(nums)
        return max(nums[n-1]*nums[n-2]*nums[n-3], nums[n-1]*nums[0]*nums[1])

class Solution1:
    def maximumProduct(self, nums: List[int]) -> int:
        m1, m2, m3 = float('-inf'), float('-inf'), float('-inf')
        mi1, mi2 = float('inf'), float('inf')
        for num in nums:
            if num > m1:
                m1, m2, m3 = num, m1, m2
            elif num > m2:
                m2, m3 = num, m2
            elif num > m3:
                m3 = num

            if num < mi1:
                mi1, mi2 = num, mi1
            elif num < mi2:
                mi2 = num
        return max(m1*m2*m3, m1*mi1*mi2)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()