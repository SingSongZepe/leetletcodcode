
class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        count = 0
        c = s[0]
        cs, ucs = 1, 0
        i, l = 1, len(s)
        s += '1' if s[-1] == '0' else '0'
        while i < l:
            if s[i] == c:
                cs += 1
                i += 1
            else:
                while i < l and s[i] != c:
                    ucs += 1
                    i += 1
                count += min(cs, ucs)
                c = '1' if c == '0' else '0'
                cs = ucs
                ucs = 0

        return count


class Solution1:
    def countBinarySubstrings(self, s: str) -> int:
        count = 0
        prev_length = 0
        current_length = 1
        for i in range(1, len(s)):
            if s[i] == s[i - 1]:
                current_length += 1
            else:
                count += min(prev_length, current_length)
                prev_length = current_length
                current_length = 1

        count += min(prev_length, current_length)

        return count

class Solution2:
    def countBinarySubstrings(self, s: str) -> int:
        if not s:
            return 0

        ans = 0
        curr = s[0]
        curr_cnt = 1
        prev = 0
        for val in s[1:]:
            if curr == val:
                curr_cnt += 1
            else:
                prev = curr_cnt
                curr = val
                curr_cnt = 1

            if curr_cnt <= prev:
                ans += 1

        return ans

def main():
    print('Hello World')

if __name__ == '__main__':
    main()