
class Solution:
    def numWays(self, s: str) -> int:
        c = s.count('1')
        if c % 3 != 0:
            return 0

        dc = c // 3

        l = len(s)
        # all zeros
        if dc == 0:
            return (l - 1) * (l - 2) // 2 % 1000000007

        # at least 3 ones
        p1 = 0
        idx1 = -1
        while p1 < dc:
            idx1 += 1
            if s[idx1] == '1':
                p1 += 1

        p2 = 0
        idx2 = l
        while p2 < dc:
            idx2 -= 1
            if s[idx2] == '1':
                p2 += 1

        s_in = s[idx1+1:idx2]
        idx3_f = s_in.find('1')
        idx3_b = s_in.rfind('1')

        return (idx3_f + 1) * (idx2 - idx1 - idx3_b - 1) % 1000000007


def main():
    print('Hello World')

if __name__ == '__main__':
    main()