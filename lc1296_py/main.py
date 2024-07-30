
from typing import List

from collections import deque
class Solution:
    def isPossibleDivide(self, nums: List[int], k: int) -> bool:
        nums.sort()
        finding = deque()

        for n in nums:
            if len(finding) == 0:
                finding.append([1, n])
                continue

            if finding[0][0] == k:
                finding.popleft()
                continue

            item = finding.popleft()
            if item[1] + 1 == n:
                item[0] += 1
                item[1] = n
            elif item[1] == n:
                finding.append([1, n])
            else:
                return False

            if item[0] < k:
                finding.append(item)

        return len(finding) == 0

def main():
    print('Hello World')

if __name__ == '__main__':
    main()