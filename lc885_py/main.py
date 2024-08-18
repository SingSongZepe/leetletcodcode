

from typing import List

class Solution:
    def spiralMatrixIII(self, rows: int, cols: int, r: int, c: int) -> List[List[int]]:
        res = [[r, c]]
        if rows == 1 and cols == 1:
            return res

        cnt = 1
        tot = rows * cols

        step = 1

        while True:
            for dx, dy, nstep in [(0, 1, step), (1, 0, step), (0, -1, step+1), (-1, 0, step+1)]:
                for _ in range(nstep):
                    r += dx
                    c += dy
                    if 0 <= r < rows and 0 <= c < cols:
                        res.append([r, c])
                        cnt += 1
                        if cnt == tot:
                            return res

            step += 2



def main():
    print('Hello World')

if __name__ == '__main__':
    main()