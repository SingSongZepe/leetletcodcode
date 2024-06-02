
from typing import List
from collections import Counter
class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        c = Counter(chars)
        l = 0
        for word in words:
            curr = c.copy()
            form = True
            for ch in word:
                if ch in curr and curr[ch] > 0:
                    curr[ch] -= 1
                else:
                    form = False
                    break
            if form:
                l += len(word)
        return l


class Solution1:
    def countCharacters(self, words: List[str], chars: str) -> int:
        c = Counter(chars)
        l = 0
        for word in words:
            for char in word:
                if c[char] < word.count(char):
                    break
            else:
                l += len(word)
        return l

def main():
    print('Hello World')

if __name__ == '__main__':
    main()