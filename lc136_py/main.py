
from typing import List
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        frequency = {}
        for num in nums:
            if num in frequency:
                frequency[num] += 1
            else:
                frequency[num] = 1

        for num in frequency:
            if frequency[num] == 1:
                return num


#### Because the problem requires us to find the single number in the list,
# we can use a dictionary to keep track of the frequency of each number in the list.
# We iterate through the list and for each number, we check if it is already in the dictionary.
# If it is, we increment its frequency by 1. If it is not, we add it to the dictionary with a frequency of 1.
# Finally, we iterate through the dictionary and return the number that has a frequency of 1.
# This is the single number in the list.
#### And every number at most appears twice
class Solution1:
    def singleNumber(self, nums: List[int]) -> int:
        answer = 0
        for num in nums:
            answer ^= num
        return answer

class Solution2:
    def singleNumber(self, nums: List[int]) -> int:
        return 2 * sum(set(nums)) - sum(nums)

from functools import reduce
class Solution3:
    def singleNumber(self, nums: List[int]) -> int:
        return reduce(lambda x, y: x ^ y, nums)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()