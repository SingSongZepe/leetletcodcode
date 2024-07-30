
class Solution:
    def findComplement(self, num: int) -> int:
        bf = 1
        while bf <= num:
            bf <<= 1
        return bf - 1 - num


def main():
    print('Hello World')

if __name__ == '__main__':
    main()