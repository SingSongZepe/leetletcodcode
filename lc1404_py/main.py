
class Solution:
    def numSteps(self, s: str) -> int:
        i = int(s, 2)
        count = 0
        while i != 1:
            if i % 2 == 0:
                i >>= 1
            else:
                i = i + 1
            count += 1
        return count

class Solution1:
    def numSteps(self, s: str) -> int:
        count = 0
        carry = False
        for i in range(len(s)-1, 0, -1):
            if (s[i] == '1' and not carry) or (s[i] == '0' and carry):
                count += 2
                carry = True
            else:
                count += 1
        if carry:
            count += 1
        return count

def main():
    print('Hello World')

if __name__ == '__main__':
    main()