

from collections import Counter

class Solution:
    def buddyStrings(self, s: str, goal: str) -> bool:
        if s == goal:
            count = Counter(s)
            if any(count[c] > 1 for c in count):
                return True
            return False

        diff_idx = -1
        for i in range(len(s)):
            if s[i] != goal[i]:
                if diff_idx > -1:
                    return s[diff_idx] == goal[i] and s[i] == goal[diff_idx] and s[i+1:] == goal[i+1:]
                diff_idx = i
        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()