

class Solution:
    def minimumPushes(self, word: str) -> int:
        
        ws = [0] * 26
        for w in word:
            ws[ord(w) - ord('a')] += 1
        
        ws.sort(reverse=True)

        return  sum(ws[:8]) + sum(ws[8:16]) * 2 + sum(ws[16:24]) * 3 + sum(ws[24:]) * 4


from collections import defaultdict

class Solution1:
    def minimumPushes(self, word: str) -> int:
        d = defaultdict(int)
        for w in word:
            d[w] += 1
        
        ws = sorted(d.values(), reverse=True)
        
        return sum(ws[:8]) + sum(ws[8:16]) * 2 + sum(ws[16:24]) * 3 + sum(ws[24:]) * 4


class Solution2:
    def minimumPushes(self, word: str) -> int:


        pass
        
def main():
    print('Hello World')

if __name__ == '__main__':
    main()