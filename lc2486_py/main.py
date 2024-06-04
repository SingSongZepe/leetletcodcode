
class Solution:
    def appendCharacters(self, s: str, t: str) -> int:
        p1 = 0
        p2 = 0
        ls = len(s)
        lt = len(t)
        while p1 < ls and p2 < lt:
            if s[p1] == t[p2]:
                p2 +=1
            p1 += 1
        return lt - p2


# good usage of iter function in python
# the c not in it process will consume the iterator until the c is found in the string
class Solution1:
    def appendCharacters(self, s: str, t: str) -> int:
        lt = len(t)
        it = iter(s)
        for idx, c in enumerate(t):
            if c not in it:
                return lt - idx
        return 0


def main():
    print('Hello World')

if __name__ == '__main__':
    main()