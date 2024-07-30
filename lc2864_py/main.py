
class Solution:
    def maximumOddBinaryNumber(self, s: str) -> str:

        zero_count = 0
        res = ''
        for i in range(len(s)):
            if s[i] == '1':
                res += '1'
            else:
                zero_count += 1

        res = res[:-1]
        res += '0' * zero_count
        res += '1'
        return res

class Solution1:
    def maximumOddBinaryNumber(self, s: str) -> str:
        c0 = s.count('0')
        c1 = s.count('1') - 1
        return '1' * c1 + '0' * c0 + '1'


def main():
    print('Hello World')

if __name__ == '__main__':
    main()