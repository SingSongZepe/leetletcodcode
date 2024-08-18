


from typing import List

from collections import Counter
import itertools

class Solution:
    def uncommonFromSentences(self, s1: str, s2: str) -> List[str]:
        st1 = set()
        st2 = set()

        for w in s1.split():
            if w in st1:
                st2.add(w)
            else:
                st1.add(w)

        for w in s2.split():
            if w in st1:
                st2.add(w)
            else:
                st1.add(w)

        
        return list(st1.difference(st2))

class Solution:
    def uncommonFromSentences(self, s1: str, s2: str) -> list[str]:
        uniques = filter(
            lambda x: x[1] == 1,
            Counter(itertools.chain(s1.split(), s2.split())).items(),
        )

        return [word for word, count in uniques]

def main():
    print('Hello World')

if __name__ == '__main__':
    main()