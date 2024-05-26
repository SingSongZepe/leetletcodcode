
from typing import List
class Solution:
    def countSquares(self, matrix: List[List[int]]) -> int:
        row=len(matrix)
        col=len(matrix[0])
        dp=[[0]*col for i in range(row)]
        for i in range(row):
            for j in range(col):
                if (i==0 or j==0) and matrix[i][j]==1:
                    dp[i][j]=1
        for i in range(1,row):
            for j in range(1,col):
                if matrix[i][j]==1:
                    dp[i][j]=1+min(dp[i-1][j-1],dp[i-1][j],dp[i][j-1])
        ans=0
        for rows in dp:
            ans+=sum(rows)
        return ans


def main():
    print('Hello World')

if __name__ == '__main__':
    main()