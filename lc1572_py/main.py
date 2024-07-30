
from typing import List
class Solution:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        return sum(mat[i][i] + mat[i][len(mat)-1-i] for i in range(len(mat))) - (mat[(len(mat)-1)//2][(len(mat)-1)//2] if len(mat) % 2 == 1 else 0)

class Solution1:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        return sum(mat[i][i] + mat[i][len(mat) - 1 - i] for i in range(len(mat)) if i != len(mat) // 2) if len(mat) % 2 == 1 else 0

def main():
    print('Hello World')

if __name__ == '__main__':
    main()