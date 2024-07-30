

from typing import List
from collections import defaultdict, deque
from datetime import datetime
class Solution:
    def alertNames(self, names: List[str], times: List[str]) -> List[str]:
        def in1hour(t1, t2):  # t2 > t1
            format = "%H:%M"
            time_format = lambda t: datetime.strptime(t, format)
            t1 = time_format(t1)
            t2 = time_format(t2)
            diff = (t2 - t1).total_seconds()
            return diff <= 3600

        times = [(idx, t) for idx, t in enumerate(times)]
        times.sort(key=lambda x: x[1])

        mp = defaultdict(deque)
        res = []

        for idx, t in times:
            if names[idx] in res:
                continue
            mp[names[idx]].append(t)
            if len(mp[names[idx]]) > 2: # 3
                if in1hour(mp[names[idx]][0], mp[names[idx]][2]):
                    res.append(names[idx])
                else:
                    mp[names[idx]].popleft()

        return sorted(res)


class Solution1:
    def alertNames(self, keyName: List[str], keyTime: List[str]) -> List[str]:
        cardTimes = defaultdict(list)
        for name, time in zip(keyName, keyTime):
            cardTimes[name].append(int(time.replace(":", "")))

        res = []
        for name, times in cardTimes.items():
            times.sort()
            for i in range(2, len(times)):
                if times[i] - times[i - 2] <= 100:
                    res.append(name)
                    break

        return sorted(res)


class Solution2:
    def alertNames(self, names: List[str], times: List[str]) -> List[str]:
        times = [(idx, int(t.replace(":", ""))) for idx, t in enumerate(times)]
        times.sort(key=lambda x: x[1])
        mp = defaultdict(list)
        res = set()
        for idx, t in times:
            if names[idx] in res:
                continue
            mp[names[idx]].append(t)
            if len(mp[names[idx]]) > 2:
                if mp[names[idx]][-1] - mp[names[idx]][-3] <= 100:
                    res.add(names[idx])

        return sorted(res)



def main():
    print('Hello World')

if __name__ == '__main__':
    main()