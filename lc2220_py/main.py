
class Solution:
    def minBitFlips(self, start: int, goal: int) -> int:
        bit_dif = 0
        count = 0
        while True:
            if start & (1 << bit_dif) ^ goal & (1 << bit_dif):
                count += 1
            bit_dif += 1
            if start < (1 << bit_dif) and goal < (1 << bit_dif):
                break
        return count

class Solution1:
    def minBitFlips(self, start: int, goal: int) -> int:
        bit_xor = start ^ goal
        count = 0
        while bit_xor > 0:
            count += 1
            bit_xor &= (bit_xor - 1)
        return count

def main():
    print('Hello World')

if __name__ == '__main__':
    main()