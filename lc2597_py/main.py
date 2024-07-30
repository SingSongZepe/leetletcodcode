

from typing import List
class Solution:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        nums.sort()

        visited = [False] * 1000
        path = []
        result = 0

        def recv(nums: List[int], from_: int):
            if from_ == len(nums):
                if len(path):
                    nonlocal result
                    result += 1
                return
            recv(nums, from_ + 1)
            if not ((nums[from_] + k < 1001 and visited[nums[from_] + k - 1]) or (nums[from_] > k and visited[nums[from_] - k - 1])):
                path.append(nums[from_])
                visited[nums[from_] - 1] = True
                recv(nums, from_ + 1)
                path.pop()
                visited[nums[from_] - 1] = False

        recv(nums, 0)
        return result


def main():
    print('Hello World')

if __name__ == '__main__':
    main()
