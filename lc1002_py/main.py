

from typing import List
class Solution:
    def commonChars(self, words: List[str]) -> List[str]:
        freq = {}
        for char in words[0]:
            freq[char] = freq.get(char, 0) + 1

        for word in words[1:]:
            for c in freq:
                if freq[c] == 0:
                    continue
                freq[c] = min(freq[c], word.count(c))

        return [c for c, count in freq.items() if count > 0 for _ in range(count)]


class Solution1:
    def commonChars(self, words: List[str]) -> List[str]:
        """
        不可能把每个word都建一个hash map。
        这题关键在于，result中其实只要确定char的最小次数

        """
        if len(words) == 1:
            return words[0]

        result = []
        chars = set(words[0])

        for char in chars:
            frequency = min([word.count(char) for word in words])
            result += frequency * [char]

        return result


def main():
    print('Hello World')

if __name__ == '__main__':
    main()