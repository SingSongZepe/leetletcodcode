
from typing import List
class Solution:
    def numOfBurgers(self, ts: int, cs: int) -> List[int]:
        diff = ts - 2 * cs
        if diff < 0 or diff % 2 != 0:
            return []
        return [diff // 2, cs - (diff // 2)] if diff // 2 <= cs else []

class Solution1:
    def numOfBurgers(self, ts: int, cs: int) -> List[int]:
        if ts < 2 * cs or ts > 4 * cs:
            return []
        if ts % 2 == 0:
            diff = ts - 2 * cs
            if diff % 2 == 0 and cs >= diff // 2:
                return [diff // 2, cs - diff // 2]
        return []

def main():
    print('Hello World')

if __name__ == '__main__':
    main()