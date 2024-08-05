

class Solution:
    def balancedStringSplit(self, s: str) -> int:
        R, L, count = 0, 0, 0
        for c in s:
            if c == 'R':
                R += 1
            else:
                L += 1
            if R == L:
                count += 1
        
        return count
        

def main():
    print('Hello World')

if __name__ == '__main__':
    main()