from typing import List


class Solution:
    def sortPeople(self, names: List[str], heights: List[int]) -> List[str]:
        return [x[0] for x in sorted([(names[idx], height) for idx, height in enumerate(heights)], key=lambda x: -x[1])]




def main():
    print('Hello World')

if __name__ == '__main__':
    main()