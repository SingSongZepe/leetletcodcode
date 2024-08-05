

from typing import List
from collections import Counter


class Solution:
    def kthDistinct(self, arr: List[str], k: int) -> str:
        
        s1, s2 = set(), set()
        
        for w in arr:
            if w in s1:
                s2.add(w)
            else:
                s1.add(w)

        for w in arr:
            if w in s2:
                continue
            k -= 1
            if k == 0:
                return w

        return ""
        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()