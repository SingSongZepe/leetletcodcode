

from typing import List

class Solution:
    def countDays(self, days: int, meetings: List[List[int]]) -> int:
        meetings.sort()
        res = 0
        last_day = 0

        for start, end in meetings:
            if start > last_day + 1:
                res += start - last_day - 1
            last_day = max(last_day, end)

        if last_day < days:
            res += days - last_day

        return res



def main():
    print('Hello World')

if __name__ == '__main__':
    main()