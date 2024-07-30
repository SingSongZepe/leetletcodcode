

from typing import List
class Solution:
    def addToArrayForm(self, nums: List[int], k: int) -> List[int]:

        carry = False
        p = len(nums) - 1
        while k > 0 and p >= 0:
            curr = k % 10
            k = k // 10
            nums[p] += curr + carry
            if nums[p] > 9:
                nums[p] -= 10
                carry = True
            else:
                carry = False
            p -= 1

        while k > 0:
            curr = k % 10 + carry
            k = k // 10
            if curr > 9:
                curr -= 10
                carry = True
            else:
                carry = False

            nums.insert(0, curr)

        while p >= 0:
            if not carry:
                break
            else:
                if nums[p] == 9:
                    nums[p] = 0
                    carry = True
                else:
                    nums[p] += 1
                    carry = False
            p -= 1

        if carry:
            nums.insert(0, 1)

        return nums


class Solution1:
    def addToArrayForm(self, nums: List[int], k: int) -> List[int]:
        carry = 0
        p = len(nums) - 1

        while k > 0 or carry:
            if p < 0:
                nums.insert(0, 0)
                p = 0

            curr = k % 10 + carry + nums[p]
            k = k // 10 if k > 0 else 0
            nums[p] = curr % 10
            carry = curr // 10
            p -= 1

        return nums

def main():
    print('Hello World')

if __name__ == '__main__':
    main()