

from typing import List

class Solution:
    def chalkReplacer(self, chalk: List[int], k: int) -> int:
        k = k % sum(chalk)
        for i in range(len(chalk)):
            if k < chalk[i]:
                return i
            k -= chalk[i]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()