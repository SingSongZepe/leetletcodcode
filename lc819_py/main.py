

from typing import List
from collections import defaultdict

class Solution:
    def mostCommonWord(self, paragraph: str, banned: List[str]) -> str:
        word = ''
        counts = defaultdict(int)
        for c in paragraph + ' ':
            if c in " !?',;.":
                if word:
                    counts[word] += 1
                    word = ''
            else:
                word += c.lower()

        banned = set(banned)
        most_common = ''
        most_count = 0
        for w, c in counts.items():
            if w not in banned and c > most_count:
                most_common = w
                most_count = c

        return most_common

from collections import Counter

import re
class Solution1:
    def mostCommonWord(self, paragraph: str, banned: List[str]) -> str:
        spec = r"[ !?',;.]+"
        paragraph = re.sub(spec, ' ', paragraph).lower()
        counts = Counter(paragraph.split())
        banned = set(banned)

        most_common = ''
        most_count = 0
        for w, c in counts.items():
            if w not in banned and c > most_count:
                most_common = w
                most_count = c

        return most_common

def main():
    print('Hello World')

if __name__ == '__main__':
    main()