
from typing import List
class Solution:
    def find132pattern(self, nums: List[int]) -> bool:
        l = len(nums)
        if l < 3:
            return False

        i = 1
        while i < l - 1:
            while i < l - 2 and nums[i+1] > nums[i]:
                i += 1
            front_min = min(nums[:i])
            rear_max = nums[i+1]
            for rear in range(i+2, l):
                if nums[rear] < nums[i]:
                    rear_max = max(rear_max, nums[rear])

            if rear_max > front_min and rear_max < nums[i]:
                return True
            i += 1

        return False

# not that good solution
class Solution1:
    def forSort(self, nums: List[int]) -> bool:
        x=len(nums) # gets length of nums
        for j in range(1,x-1): # iterating possible j
            if nums[j-1]==nums[j] : # skips duplicate j comparing with previous element
                continue
            for i in range(max(0,j-4),j): # iterating possible i such that 0<i<j
                if nums[i]>nums[j]: # skipping nums[i]>nums[j] case
                    continue
                for k in range(j+1,min(j+8,x)): # iterating possible values of K such that j<k<len(nums)
                    if nums[i]<nums[k]<nums[j]: #checking 132 condition
                        return True
        return False

    def find132pattern(self, nums: List[int]) -> bool:
        if len(nums) > 10: # if list too big
            for i in [10, 30, 60, 90, 120, 150, len(nums)]: # we take it piecewise and slowly increment it to full length
                nums = nums[:i]
                if self.forSort(nums):
                    return True
        return self.forSort(nums) #call our condition finder


class Solution2:
    def find132pattern(self, nums: List[int]) -> bool:

        #  this is an example of monotonic stack ds
        # this actually maintains either decreasing or increasing elements in a stack
        # consider last 2 positions if the
        # consider eg ::    1 5 0 3 4
        '''
            initially 4 in stack
            the since 3<4 3 will add then 0 will add into the stack
            now 5 since 5>0 pop all elements that are less than the 5
            and update the mini2 as the max element that is popped out

            after this operation
            mini2 = 4
            in stack is 5

            the next num which is less than mini2 will form the series
            here   1 --> 5 --> 4 will form the patten

        '''

        n = len(nums)
        st = []
        mini2 = float('-inf')

        for i in range(n - 1, -1, -1):
            if i == n - 1:
                st.append(nums[i])
            if nums[i] < mini2:
                return True
            if nums[i] < st[-1]:
                st.append(nums[i])
            elif nums[i] > st[-1]:
                while (len(st) > 0 and nums[i] > st[-1]):
                    mini2 = st[-1]
                    st.pop()
                st.append(nums[i])

        return False

class Solution3:
    def find132pattern(self, nums: List[int]) -> bool:
        n = len(nums)
        monostack = []

def main():
    print('Hello World')

if __name__ == '__main__':
    main()