
from typing import List
class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        s = sum(nums[:k])
        m = s
        for i in range(k, len(nums)):
            s += nums[i] - nums[i-k]
            m = max(m, s)
        return m / k



def main():
    print('Hello World')

if __name__ == '__main__':
    main()