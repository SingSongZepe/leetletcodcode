

from typing import List

class Solution:
    def projectionArea(self, grid: List[List[int]]) -> int:
        x = sum(1 if h > 0 else 0 for row in grid for h in row)
        z = sum(max(h for h in row) for row in grid)
        l = len(grid)
        for row in grid[1:]:
            for i in range(l):
                if row[i] > grid[0][i]:
                    grid[0][i] = row[i]
        y = sum(grid[0])

        return x + y + z


def main():
    print('Hello World')

if __name__ == '__main__':
    main()