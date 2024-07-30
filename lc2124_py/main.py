

class Solution:
    def checkString(self, s: str) -> bool:
        b = False

        for c in s:
            if c == 'b':
                b = True
            elif b and c == 'a':
                return False
        return True

class Solution1:
    def checkString(self, s: str) -> bool:
        return sorted(s) == list(s) 


def main():
    print('Hello World')

if __name__ == '__main__':
    main()