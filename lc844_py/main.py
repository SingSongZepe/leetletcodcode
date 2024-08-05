

class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        
        p1 = 0
        for i in range(len(s)):
            if s[i] == '#':
                p1 = max(p1-1, 0)
            else:
                s = s[:p1] + s[i] + s[p1+1:]
                p1 += 1
        
        p2 = 0
        for i in range(len(t)):
            if t[i] == '#':
                p2 = max(p2-1, 0)
            else:
                t = t[:p2] + t[i] + t[p2+1:]
                p2 += 1
        
        return p1 == p2 and s[:p1] == t[:p2]


class Solution1:
    def backspaceCompare(self, s: str, t: str) -> bool:

        def helper(s):
            
            res = ''
            for c in s:
                if c == '#':
                    res = res[:-1]
                else:
                    res += c
            return res
        
        return helper(s) == helper(t)
        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()