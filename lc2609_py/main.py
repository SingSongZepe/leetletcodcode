


class Solution:
    def findTheLongestBalancedSubstring(self, s: str) -> int:

        zeros = 0
        m = 0
        curr = 0
        down = False
        l = len(s)

        for idx, c in enumerate(s):
            if c == '0':
                if down:
                    m = max(m, 2 * min(curr, zeros))
                    zeros = 1
                    curr = 0
                    down = False
                else:
                    zeros += 1
            else:
                curr += 1
                if idx == l - 1 or s[idx+1] == '0':
                    down = True

        return max(m, 2 * min(curr, zeros))

class Solution1:
    def findTheLongestBalancedSubstring(self, s: str) -> int:

        zeros = 0
        m = 0
        curr = 0
        down = False
        l = len(s)

        for idx in range(l):
            if s[idx] == '0':
                if down:
                    m = max(m, 2 * min(curr, zeros))
                    zeros = 1
                    curr = 0
                    down = False
                else:
                    zeros += 1
            else:
                curr += 1
                if idx == l - 1 or s[idx+1] == '0':
                    down = True

        return max(m, 2 * min(curr, zeros))

def main():
    print('Hello World')

if __name__ == '__main__':
    main()