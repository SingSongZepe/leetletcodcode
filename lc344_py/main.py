
from typing import List
class Solution:
    def reverseString(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        s.reverse()

class Solution1:
    def reverseString(self, s: List[str]) -> None:
        for i in range(len(s)//2):
            s[i], s[len(s)-i-1] = s[len(s)-i-1], s[i]

class Solution2:
    def reverseString(self, s: List[str]) -> None:
        s[:] = s[::-1]

def main():
    print('Hello World')

if __name__ == '__main__':
    main()