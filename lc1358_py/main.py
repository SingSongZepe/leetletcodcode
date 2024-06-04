
class Solution:
    def numberOfSubstrings(self, s: str) -> int:
        def to(c: str) -> int:
            return ord(c) - 97

        mp = [0] * 3
        ls = len(s)

        p = 0
        l = 0
        while p < ls:
            mp[to(s[p])] += 1
            p += 1
            if all(mp):
                break

        count = 0
        while l < p - 1 and all(mp):
            mp[to(s[l])] -= 1
            l += 1
        count += l
        while p < ls:
            mp[to(s[p])] += 1
            while l < p - 1 and all(mp):
                mp[to(s[l])] -= 1
                l += 1
            count += l
            p += 1
        return count

class Solution1:
    def numberOfSubstrings(self, s: str) -> int:
        def to(c: str) -> int:
            return 0 if c == 'a' else 1 if c == 'b' else 2

        char_freq = [0] * 3
        count = 0
        l = 0

        for p, c in enumerate(s):
            char_freq[0 if c == 'a' else 1 if c == 'b' else 2] += 1

            while all(char_freq):
                count += len(s) - p
                nc = s[l]
                char_freq[0 if nc == 'a' else 1 if nc == 'b' else 2] -= 1
                l += 1

        return count

class Solution2:
    def numberOfSubstrings(self, s: str) -> int:
        a = b = c = 0                        # counter for letter a/b/c
        ans, i, n = 0, 0, len(s)             # i: slow pointer
        for j, letter in enumerate(s):       # j: fast pointer
            if letter == 'a':
                a += 1         # increment a/b/c accordingly
            elif letter == 'b':
                b += 1
            else:
                c += 1
            while a > 0 and b > 0 and c > 0: # if all of a/b/c are contained, move slow pointer
                ans += n-j                   # count possible substr, if a substr ends at j, then there are n-j substrs to the right that are containing all a/b/c
                if s[i] == 'a':
                    a -= 1       # decrement counter accordingly
                elif s[i] == 'b':
                    b -= 1
                else:
                    c -= 1
                i += 1                       # move slow pointer
        return ans


def main():
    print('Hello World')

if __name__ == '__main__':
    main()