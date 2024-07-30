
from collections import Counter
class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        return sorted(Counter(word1).values()) == sorted(Counter(word2).values()) if set(word1) == set(word2) else False if len(word1) == len(word2) else False




def main():
    print('Hello World')

    c = Counter('counter')
    print(sorted(c.values()))

if __name__ == '__main__':
    main()