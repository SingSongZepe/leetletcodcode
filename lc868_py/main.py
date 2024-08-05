

class Solution:
    def binaryGap(self, n: int) -> int:
        
        m, d = 0, -1

        while n:
            if n & 1:
                if d == -1:
                    d = 1
                else:
                    m = max(m, d)
                    d = 1
            else:
                if d != -1:
                    d += 1
            n >>= 1

        return m



        
def main():
    print('Hello World')

if __name__ == '__main__':
    main()