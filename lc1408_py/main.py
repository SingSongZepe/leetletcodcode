

from typing import List
class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        def add(word: str):
            node = trie
            for c in word:
                node = node.setdefault(c, {})

        def get(word: str) -> bool:
            node = trie
            for c in word:
                if (node := node.get(c)) is None: return False
            return True

        words.sort(key=len, reverse=True)
        trie, result = {}, []
        for word in words:
            if get(word):
                result.append(word)
            for i in range(len(word)):
                add(word[i:])
        return result

class Solution1:
    def stringMatching(self, words: List[str]) -> List[str]:
        words.sort(key=len, reverse=True)
        s = words[0] + ' '
        res = []
        for i in range(1, len(words)):
            if words[i] in s:
                res.append(words[i])
            s += words[i] + ' '
        return res


class Solution2:
    def stringMatching(self, words: List[str]) -> List[str]:
        words.sort(key=len, reverse=True)
        res = []
        s = ' '.join(words)
        for w in words:
            if s.count(w) > 1:
                res.append(w)
        return res

def main():
    print('Hello World')

if __name__ == '__main__':
    main()