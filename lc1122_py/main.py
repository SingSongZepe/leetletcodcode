
from typing import List
class Solution:
    def relativeSortArray(self, arr1: List[int], arr2: List[int]) -> List[int]:
        l = len(arr2)
        mp = {}
        for idx, num in enumerate(arr2):
            mp[num] = idx

        def key_func(x):
            if x in mp:
                return mp[x]
            else:
                return l + x

        arr1.sort(key=key_func)

        return arr1



def main():
    print('Hello World')

if __name__ == '__main__':
    main()