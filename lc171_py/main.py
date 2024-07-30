
class Solution:
    def titleToNumber(self, columnTitle: str) -> int:
        column = 0
        l = len(columnTitle)
        for idx in range(l-1, -1, -1):
            column += (ord(columnTitle[idx]) - 64) * (26 ** (l-1-idx))
        return column

def main():
    print('Hello World')

if __name__ == '__main__':
    main()