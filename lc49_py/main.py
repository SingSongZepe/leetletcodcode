
from typing import List

from collections import defaultdict
# from itertools import reduce

class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        def cnt(s: str):
            lst = [0] * 26
            for c in s:
                lst[ord(c) - ord('a')] += 1
            return tuple(lst)


        mp = defaultdict(list)

        for s in strs:
            mp[cnt(s)].append(s)

        return list(mp.values())

        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()