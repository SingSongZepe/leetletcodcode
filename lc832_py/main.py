

from typing import List

class Solution:
    def flipAndInvertImage(self, image: List[List[int]]) -> List[List[int]]:
        l = len(image[0])
        for i, curr in enumerate(image):
            tmp = curr.copy()
            for j in range(l):
                image[i][j] = 1 - tmp[l-j-1]
        return image

class Solution1:
    def flipAndInvertImage(self, image: List[List[int]]) -> List[List[int]]:
        return [[1-i for i in row[::-1]] for row in image]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()