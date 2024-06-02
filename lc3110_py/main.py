

class Solution:
    def scoreOfString(self, s: str) -> int:
        def abs(x: int) -> int:
            return x if x >= 0 else -x

        sum = 0
        for i in range(len(s)-1):
            sum += abs(ord(s[i+1]) - ord(s[i]))

        return sum


def main():
    print('Hello World')

if __name__ == '__main__':
    main()