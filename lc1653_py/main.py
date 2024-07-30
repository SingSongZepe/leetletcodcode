


class Solution:
    def minimumDeletions(self, s: str) -> int:
        c = s[1:].count('a')
        m = c
        for i in range(1, len(s)):
            if s[i-1] == 'b':
                c += 1
            if s[i] == 'a':
                c -= 1
            m = min(m, c)
        return m

class Solution1:
    def minimumDeletions(self, s: str) -> int:
        m, c = 0, 0
        for i in s:
            if i == 'b':
                c += 1
            elif c:
                c -= 1
                m += 1
        return m
    
def main():
    print('Hello World')

if __name__ == '__main__':
    main()