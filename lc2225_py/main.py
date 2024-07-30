
from typing import List

class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        failed_0, failed_1, failed_more = set(), set(), set()

        for w, l in matches:
            if w not in failed_1 and w not in failed_more:
                failed_0.add(w)

            if l in failed_1:
                failed_1.remove(l)
                failed_more.add(l)
            elif l in failed_0:
                failed_0.remove(l)
                failed_1.add(l)
            elif l in failed_more:
                continue
            else:
                failed_1.add(l)

        return [sorted(failed_0), sorted(failed_1)]

from collections import defaultdict
class Solution1:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        mp = defaultdict(int)
        for w, l in matches:
            mp[w] += 0
            mp[l] += 1

        result = [[], []]
        for k, v in mp.items():
            if v == 0:
                result[0].append(k)
            elif v == 1:
                result[1].append(k)

        return [sorted(result[0]), sorted(result[1])]

def main():
    print('Hello World')

if __name__ == '__main__':
    main()