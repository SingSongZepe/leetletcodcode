
from typing import List

class Solution:
    def minHeightShelves(self, books: List[List[int]], shelfWidth: int) -> int:
        
        l = len(books)
        dp = [0] * (l+1)

        for i, (thickness, height) in enumerate(books, 1):
            
            # if new layer
            dp[i] = dp[i-1] + height
            
            # if with some previous book in the same layer
            for j in range(i - 2, -1, -1):

                thickness += books[j][0]

                if thickness > shelfWidth:
                    break
                
                height = max(height, books[j][1])

                dp[i] = min(dp[i], dp[j] + height)

        return dp[l]

def main():
    print('Hello World')

if __name__ == '__main__':
    main()