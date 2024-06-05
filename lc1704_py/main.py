


class Solution:
    def halvesAreAlike(self, s: str) -> bool:
        return True if sum(1 for c in s[:len(s)//2] if c in 'aeiouAEIOU') == sum(1 for c in s[len(s)//2:] if c in 'aeiouAEIOU') else False



def main():
    print('Hello World')

if __name__ == '__main__':
    main()