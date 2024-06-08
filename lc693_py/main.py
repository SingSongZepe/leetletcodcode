
class Solution:
    def hasAlternatingBits(self, n: int) -> bool:

        bit = n & 1
        n >>= 1
        while n > 0:
            if n & 1 == bit:
                return False
            bit = n & 1
            n >>= 1
        return True

class Solution1:
    def hasAlternatingBits(self, n: int) -> bool:
        return '00' not in bin(n) and '11' not in bin(n)




def main():
    print('Hello World')

if __name__ == '__main__':
    main()