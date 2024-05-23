

class Solution:
    def reversePrefix(self, word: str, ch: str) -> str:
        try:
            index = word.index(ch)
            return word[:index+1][::-1] + word[index+1:]
        except ValueError:
            return word

class Solution1:
    def reversePrefix(self, word: str, ch: str) -> str:
        if idx := word.find(ch):
            return word[:idx+1][::-1] + word[idx+1:]
        else:
            return word

class Solution2:
    def reversePrefix(self, word: str, ch: str) -> str:
        return word[:idx + 1][::-1] + word[idx + 1:] if (idx := word.find(ch)) else word


def main():
    print('Hello World')

if __name__ == '__main__':
    main()