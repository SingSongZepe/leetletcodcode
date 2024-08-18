

from typing import List

class Solution(object):
    def maxDistance(self, arrays):
        smallest = arrays[0][0]
        biggest = arrays[0][-1]
        max_distance = 0

        for arr in arrays[1:]:
            max_distance = max(max_distance, abs(arr[-1] - smallest), abs(biggest - arr[0]))
            smallest = min(smallest, arr[0])
            biggest = max(biggest, arr[-1])

        return max_distance



def main():
    print('Hello World')

if __name__ == '__main__':
    main()