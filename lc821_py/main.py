

from typing import List

class Solution:
    def shortestToChar(self, s: str, c: str) -> List[int]:
        prev = -10001
        csp = []
        for idx, ch in enumerate(s):
            if ch == c:
                csp.append(idx)
        lst = csp.pop(0)

        res = []
        for idx, ch in enumerate(s):
            if ch == c:
                res.append(0)
                prev = lst
                if csp:
                    lst = csp.pop(0)
                else:
                    lst = 10001
            else:
                res.append(min(lst - idx, idx - prev))

        return res



def main():
    print('Hello World')

if __name__ == '__main__':
    main()