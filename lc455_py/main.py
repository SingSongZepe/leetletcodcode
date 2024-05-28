
from typing import List
class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        g.sort()
        s.sort()

        gi = 0

        for si in range(len(s)):
            if s[si] >= g[gi]:
                gi += 1
            if gi == len(g):
                break
        return gi

def main():
    print('Hello World')

if __name__ == '__main__':
    main()