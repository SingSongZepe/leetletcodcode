
from typing import List
class Solution:
    def construct2DArray(self, original: List[int], m: int, n: int) -> List[List[int]]:
        return [original[i*n:i*n+n] for i in range(m)] if m * n == len(original) else []


def main():
    print('Hello World')

if __name__ == '__main__':
    main()