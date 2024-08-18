
from typing import List

class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        dp: List[int] = points[0]
        n = len(points[0])

        for pts in points[1:]:
            lm = [dp[0]] * n
            rm = [dp[-1]-n+1] * n
            for i in range(1, n):
                lm[i] = max(dp[i] + i, lm[i-1])
            for i in range(n-2, -1, -1):
                rm[i] = max(dp[i] - i, rm[i+1])
            
            for i in range(n):
                dp[i] = max(lm[i] - i, rm[i] + i) + pts[i]
            
        return max(dp)


class Solution1:  
    def maxPoints(self, points: List[List[int]]) -> int:  
        if not points:  
            return 0  
        
        n = len(points[0])  
        dp: List[int] = points[0]   

        for pts in points[1:]:  
            new_dp = [0] * n  
            
            
            left_max = dp[0]  
            new_dp[0] = left_max + pts[0]  
            for j in range(1, n):  
                left_max = max(left_max, dp[j] + j)  
                new_dp[j] = left_max - j + pts[j]  
                
            right_max = dp[-1] - (n - 1)  
            new_dp[-1] = max(new_dp[-1], right_max + (n - 1) + pts[-1])  
            for j in range(n - 2, -1, -1):  
                right_max = max(right_max, dp[j] - j)  
                new_dp[j] = max(new_dp[j], right_max + j + pts[j])  
            
            dp = new_dp  

        return max(dp) 

def main():
    print('Hello World')

if __name__ == '__main__':
    main()