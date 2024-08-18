

from typing import List

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        mx = prices[-1]
        mpf = 0

        for i in range(len(prices)-2, -1, -1):
            mx = max(mx, prices[i])
            mpf = max(mpf, mx - prices[i])
        

        return mpf

class Solution1:
    def maxProfit(self, prices: List[int]) -> int:
        buy = 10001
        mpf = 0

        for pr in prices:
            if pr < buy:
                buy = pr
            else:
                mpf = max(mpf, pr - buy)

        return mpf
        


def main():
    print('Hello World')

if __name__ == '__main__':
    main()