

from typing import List
from collections import Counter
class Solution:
    def findCenter(self, edges: List[List[int]]) -> int:
        for k, c in Counter(edges[0] + edges[1]).items():
            if c == 2:
                return k


def main():
    print('Hello World')

if __name__ == '__main__':
    main()