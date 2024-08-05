

from typing import List

class Solution:
    def largeGroupPositions(self, s: str) -> List[List[int]]:
        
        cnt, res = 1, []

        for i, c in enumerate(s[1:]+'N', start=1):
            if c == s[i-1]:
                cnt += 1
            else:
                if cnt >= 3:
                    res.append([i-cnt, i-1])
                cnt = 1

        return res

class Solution1:
    def largeGroupPositions(self, s: str) -> List[List[int]]:
        
        cnt, res, s = 1, [], s+'N'

        for i in range(1, len(s)):
            if s[i] == s[i-1]:
                cnt += 1
            else:
                if cnt >= 3:
                    res.append([i-cnt, i-1])
                cnt = 1

        return res



def main():
    print('Hello World')

if __name__ == '__main__':
    main()