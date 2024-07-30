

from typing import List
class Solution:
    def applyOperations(self, nums: List[int]) -> List[int]:
        res = []
        i, l = 0, len(nums)
        nums.append(0)
        while i < l:
            if nums[i] == 0:
                pass
            elif nums[i] == nums[i+1]:
                res.append(2 * nums[i])
                i += 1
            else:
                res.append(nums[i])
            i += 1
        res.extend([0 for _ in range(l - len(res))])
        return res

def main():
    print('Hello World')

if __name__ == '__main__':
    main()