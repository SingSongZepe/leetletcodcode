

import heapq


class BinaryHeap:
    def __init__(self):
        self.heap = []

    def push(self, item):
        heapq.heappush(self.heap, (item[0], -item[1]))

    def pop(self):
        return heapq.heappop(self.heap)

    def sort(self, key):
        self.heap.sort(key=key)


# O(nlogn) time, middle solution
class Solution:
    def clearStars(self, s: str) -> str:
        bh = BinaryHeap()
        for i, c in enumerate(s):
            if c == '*':
                bh.pop()
            else:
                bh.push((c, i))

        bh.sort(key=lambda x: -x[1])
        return ''.join([c for c, _ in bh.heap])

from heapq import heappush, heappop
from collections import defaultdict

# O(n) time, better solution
class Solution1:
    def clearStars(self, s: str) -> str:
        heap = []
        charIndex = defaultdict(list)
        s = list(s)
        for i, c in enumerate(s):
            if c == "*":
                toRemove = heap[0]
                s[charIndex[toRemove].pop()] = ""
                if not charIndex[toRemove]:
                    heappop(heap)
                s[i] = ""
            else:
                if len(charIndex[c]) == 0:
                    heappush(heap, c)
                charIndex[c].append(i)
        return "".join(s)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()