
from typing import List
from collections import Counter, deque
class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        c = Counter(hand)
        hand.sort()

        for v in hand:
            if c[v] == 0:
                continue

            for i in range(groupSize):
                if v + i not in c or c[v + i] == 0:
                    return False
                c[v + i] -= 1

        if any(c.values()):
            return False
        else:
            return True


class Solution1:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:

        hand.sort()

        dq = deque([])

        for card in hand:
            if len(dq) > 0:
                if dq[-1][1] == card - 1:
                    item = dq.pop()
                    item[0] += 1
                    item[1] = card
                elif dq[-1][1] == card:
                    item = [1, card]
                else:
                    return False
            else:
                item = [1, card]

            if item[0] < groupSize:
                dq.appendleft(item)

        return len(dq) == 0

def main():
    print('Hello World')

if __name__ == '__main__':
    main()