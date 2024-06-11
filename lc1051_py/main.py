

from typing import List
class Solution:
    def heightChecker(self, heights: List[int]) -> int:
        return sum(1 for i in range(len(heights)) if heights[i] != sorted(heights)[i])

        pass

def main():
    print('Hello World')

if __name__ == '__main__':
    main()