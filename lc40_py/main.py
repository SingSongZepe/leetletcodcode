

from typing import List

class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        res = []
        path = []

        def combine(start, target):
            if target == 0:
                res.append(path.copy())
                return
            for i in range(start, len(candidates)):
                if i != start and candidates[i] == candidates[i-1]:
                    continue
                if candidates[i] > target:
                    break
                path.append(candidates[i])
                combine(i+1, target-candidates[i])
                path.pop()

        combine(0, target)
        return res


def main():
    print('Hello World')

if __name__ == '__main__':
    main()