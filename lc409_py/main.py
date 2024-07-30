
class Solution:
    def longestPalindrome(self, s: str) -> int:
        def to(c: str):
            return ord(c) - 97 if ord(c) > 96 else ord(c) - 39

        mp = [0] * 52
        for c in s:
            mp[to(c)] += 1

        odd = False
        count = 0
        for co in mp:
            count += co // 2 * 2
            if not odd and co % 2 == 1:
                odd = True

        return count + odd


class Solution1:
    def longestPalindrome(self, s: str) -> int:
        ss = set()
        for letter in s:
            if letter not in ss:
                ss.add(letter)
            else:
                ss.remove(letter)
        if len(ss) != 0:
            return len(s) - len(ss) + 1
        else:
            return len(s)

class Solution2:
    def longestPalindrome(self, s: str) -> int:
        odds = set()
        for c in s:
            if c in odds:
                odds.remove(c)
            else:
                odds.add(c)

        if len(odds) == 0:
            return len(s)
        else:
            return len(s) - len(odds) + 1


def main():
    print('Hello World')

if __name__ == '__main__':
    main()