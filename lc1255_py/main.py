
from typing import List
from collections import Counter
class Solution:
    def maxScoreWords(self, words: List[str], letters: List[str], scores: List[int]) -> int:

        max_score = 0
        counter = Counter(letters)

        def recv(words: List[str], path: List[str], from_: int):
            if from_ == len(words):
                nonlocal max_score
                max_score = max(max_score, get_score(path))
                return

            recv(words, path, from_+1)
            path.append(words[from_])
            recv(words, path, from_+1)
            path.pop()

        # path maybe empty
        def get_score(path: List[str]) -> int:
            str = ''.join(path)
            if len(str) > len(letters):
                return 0
            c = Counter(str)
            if c <= counter:
                return sum([scores[ord(ch)-ord('a')] for ch in str])
            else:
                return 0

        recv(words, [], 0)
        return max_score

class Solution1:
    def maxScoreWords(self, words: List[str], letters: List[str], score: List[int]) -> int:
        letterCount = [0] * 26
        for l in letters:
            letterCount[ord(l) - ord('a')] += 1
        return self.dfs(words, score, letterCount, 0)

    def dfs(self, words: List[str], score: List[int], letterCount: List[int], index: int) -> int:
        if index == len(words):
            return 0
        skipScore = self.dfs(words, score, letterCount, index + 1)
        wordScore = 0
        newLetterCount = letterCount[:]
        valid = True
        for c in words[index]:
            if newLetterCount[ord(c) - ord('a')] == 0:
                valid = False
                break
            newLetterCount[ord(c) - ord('a')] -= 1
            wordScore += score[ord(c) - ord('a')]
        takeScore = 0
        if valid:
            takeScore = wordScore + self.dfs(words, score, newLetterCount, index + 1)
        return max(skipScore, takeScore)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()