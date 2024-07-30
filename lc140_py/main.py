
from typing import List
from collections import Counter
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        c = Counter(wordDict)
        result = []

        def recv(s: str, from_: int, path: List[str]):
            if from_ == len(s):
                result.append(path)
                return
            for i in range(from_, len(s)+1):
                if s[from_:i] in c:
                    recv(s, i, path+[s[from_:i]])

        recv(s, 0, [])
        return [' '.join(path) for path in result]




def main():
    print('Hello World')

if __name__ == '__main__':
    main()