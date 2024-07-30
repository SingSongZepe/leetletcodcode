
from typing import List
class Solution:
    def makeEqual(self, words: List[str]) -> bool:
        table = [0] * 26
        s = ''.join(words)
        l = len(words)
        del words
        for char in s:
            table[ord(char) - ord('a')] += 1
        return all(count % l == 0 for count in table)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()