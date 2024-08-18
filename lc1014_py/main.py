

from typing import List

class Solution:
    def maxScoreSightseeingPair(self, values: List[int]) -> int:

        mx = values[0]
        mpf = values[0] + values[1] - 1

        for i in range(1, len(values)):
            mpf = max(mpf, mx + values[i] - i)
            mx = max(mx, values[i] + i)

        return mpf

def main():
    print('Hello World')

if __name__ == '__main__':
    main()