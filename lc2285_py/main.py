
from typing import List

class Solution:
    def maximumImportance(self, n: int, roads: List[List[int]]) -> int:
        ln = [0] * n
        for conn in roads:
            ln[conn[0]] += 1
            ln[conn[1]] += 1

        ln.sort()
        sum = 0
        for poi in range(1, n+1):
            sum += poi * ln[poi-1]
        return sum

def main():
    print('Hello World')

if __name__ == '__main__':
    main()