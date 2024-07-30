
class Solution:
    def minSteps(self, s: str, t: str) -> int:
        mp = [0] * 26

        for i in range(len(s)):
            mp[ord(s[i])-97] += 1
            mp[ord(t[i])-97] -= 1

        return sum(abs(x) for x in mp) // 2

from collections import defaultdict
class Solution1:
    def minSteps(self, s: str, t: str) -> int:
        mp = defaultdict(int)

        for i in range(len(s)):
            mp[s[i]] += 1
            mp[t[i]] -= 1

        return sum(abs(x) for x in mp.values()) // 2

class Solution2:
    def minSteps(self, s: str, t: str) -> int:
        working = set(s)
        count = 0
        for letter in working:
            count_main = s.count(letter)
            count_minor = t.count(letter)
            count += abs(count_main - count_minor)
        return count // 2


def main():
    print('Hello World')

if __name__ == '__main__':
    main()