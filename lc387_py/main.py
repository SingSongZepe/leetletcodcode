

class Solution:
    # I was considering that, the meaning of the problem is to
    # find the first unique character (that is its neighboring characters is not the same),
    # for example, in the string "leetcode", the first unique character is 'l'
    # and its neighboring characters are 'eebccb' is 'b'
    def firstUniqChar(self, s: str) -> int:
        repeated = False

        for i in range(len(s)-1):
            if not repeated and s[i] != s[i+1]:
                return i
            elif repeated and s[i] != s[i+1]:
                repeated = False
            else:
                repeated = True


class Solution1:
    def firstUniqChar(self, s: str) -> int:
        counters = [0] * 26
        for c in s:
            counters[ord(c) - 97] += 1

        for c in s:
            if counters[ord(c) - 97] == 1:
                return s.find(c)
        return -1


from collections import Counter
class Solution2:
    def firstUniqChar(self, s: str) -> int:
        char = Counter(s)

        for i, val in enumerate(s):
            if char[val] == 1:
                return i
        return -1


# may be the best algo
class Solution3:
    def firstUniqChar(self, s: str) -> int:
        text = 'abcdefghijklmnopqrstuvwxyz'
        idx = 10 ** 5

        for i in text:
            x = s.find(i)
            if x != -1 and x == s.rfind(i):
                idx = min(idx, x)
        return idx if idx != 10 ** 5 else -1


def main():
    print('Hello World')

if __name__ == '__main__':
    main()