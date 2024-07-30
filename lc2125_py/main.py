
from typing import List
class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        pre = 0
        count = 0
        for row in bank:
            curr = row.count('1')
            if curr > 0:
                count += pre * curr
                pre = curr
        return count



def main():
    print('Hello World')

if __name__ == '__main__':
    main()