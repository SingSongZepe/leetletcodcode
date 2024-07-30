

class Solution:
    def addDigits(self, num: int) -> int:
        return 0 if num == 0 else (num - 1) % 9 + 1


        
def main():
    print('Hello World')

if __name__ == '__main__':
    main()