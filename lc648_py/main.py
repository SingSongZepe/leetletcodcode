
from typing import List
class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        d = set(dictionary)
        mi = min(len(w) for w in d)
        ma = max(len(w) for w in d)
        words = sentence.split()
        res = []
        for w in words:
            for i in range(mi, min(ma, len(w)) + 1):
                if w[:i] in d:
                    w = w[:i]
                    break
            res.append(w)

        return ' '.join(res)


class Solution1:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        d = {w: len(w) for w in dictionary}
        mi, ma = min(d.values()), max(d.values())
        wrds = sentence.split()
        res = []
        for w in wrds:
            c = w
            for i in range(mi, min(ma, len(w)) + 1):
                ss = w[:i]
                if ss in d:
                    c = ss
                    break
            res.append(c)
        return " ".join(res)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()