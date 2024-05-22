
from typing import List
class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        starts = []
        gain = []
        for i in range(len(gas)):
            if gas[i] >= cost[i]:
                starts.append(i)
            gain.append(gas[i] - cost[i])

        for start in starts:
            tank = 0
            break_flag = False
            for i in range(start, len(gas)):
                tank += gain[i]
                if tank < 0:
                    break_flag = True

            if break_flag:
                continue

            for i in range(0, start):
                tank += gain[i]
                if tank < 0:
                    break

            if break_flag:
                continue

            if tank >= 0:
                return start
        return -1


class Solution1:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        if sum(gas) < sum(cost):
            return -1
        tank = 0
        start = 0
        for i in range(len(gas)):
            tank += gas[i] - cost[i]
            if tank < 0:
                start = i + 1
                tank = 0
        return start

def main():
    print('Hello World')

if __name__ == '__main__':
    main()