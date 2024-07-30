
from typing import List
class Solution:
    def maximumValue(self, strs: List[str]) -> int:
        m = 0
        for s in strs:
            if s.isnumeric():
                m = max(m, int(s))
            else:
                m = max(len(s), m)
        return m


def main():
    print('Hello World')

if __name__ == '__main__':
    main()