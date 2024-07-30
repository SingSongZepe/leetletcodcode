

from typing import List
class Solution:
    def candy(self, ratings: List[int]) -> int:
        n, given = len(ratings), [1] * len(ratings)

        # look left
        for i in range(n - 1):
            if ratings[i] < ratings[i + 1]:
                given[i + 1] = max(given[i + 1], given[i] + 1)

        # look right
        for i in range(n - 2, -1, -1):
            if ratings[i] > ratings[i + 1]:
                given[i] = max(given[i], given[i + 1] + 1)

        return sum(given)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()