

from typing import List
class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        v = 0
        for op in operations:
            if op[1] == '+':
                v += 1
            else:
                v -= 1
        return v

class Solution1:
    def finalValueAfterOperations(self, ops: List[str]) -> int:
        return sum(1 if op[1] == '+' else -1 for op in ops)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()