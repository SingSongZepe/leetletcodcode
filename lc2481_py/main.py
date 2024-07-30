
class Solution:
    def numberOfCuts(self, n: int) -> int:
        return (n if n & 1 else n // 2) if n > 1 else 0

def main():
    print('Hello World')

if __name__ == '__main__':
    main()