
from typing import List
class Solution:
    def numberOfLines(self, widths: List[int], s: str) -> List[int]:

        curr_width = 0
        lines = 1
        for c in s:
            curr_width += widths[ord(c) - 97]
            if curr_width > 100:
                curr_width = widths[ord(c) - 97]
                lines += 1

        return [lines, curr_width]

def main():
    print('Hello World')

if __name__ == '__main__':
    main()