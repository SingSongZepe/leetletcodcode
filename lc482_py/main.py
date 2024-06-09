
from collections import deque
class Solution:
    def licenseKeyFormatting(self, s: str, k: int) -> str:
        q = []
        for char in s:
            if char == '-':
                continue
            q.append(char.upper())

        p = 0
        result = ''
        while q:
            if p == k:
                p = 0
                result = '-' + result
            result = q.pop() + result
            p += 1

        return result


class Solution1:
    def licenseKeyFormatting(self, s: str, k: int) -> str:
        s = ''.join(s.upper().split('-'))
        res = []

        i = len(s)
        while i-k >= 0:
            res.append(s[i-k:i])
            i -= k
        if i > 0:
            res.append(s[:i])

        return '-'.join(res[::-1])


def main():
    print('Hello World')

if __name__ == '__main__':
    main()