

from typing import List
class Solution:
    def findMatrix(self, nums: List[int]) -> List[List[int]]:
        mp = [0] * 201
        res = [[]]
        for n in nums:
            if mp[n] == 0:
                mp[n] = 1
                res[0].append(n)
            else:
                if len(res) > mp[n]:
                    res[mp[n]].append(n)
                else:
                    res.append([n])
                mp[n] += 1
        return res

class Solution1:
    def findMatrix(self, nums: List[int]) -> List[List[int]]:
        mp = {}
        res = [[]]
        for n in nums:
            if n not in mp:
                mp[n] = 1
                res[0].append(n)
            else:
                if len(res) > mp[n]:
                    res[mp[n]].append(n)
                else:
                    res.append([n])
                mp[n] += 1
        return res

def main():
    print('Hello World')

if __name__ == '__main__':
    main()