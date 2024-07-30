
from typing import List

class Solution:
    def subarraysDivByK(self, nums: List[int], k: int) -> int:
        count = 0
        prefix_sum = 0
        prefix_map = {0: 1}

        for num in nums:
            prefix_sum += num
            mod = prefix_sum % k
            # if mod < 0:
            #     mod += k
            if mod in prefix_map:
                count += prefix_map[mod]
                prefix_map[mod] += 1
            else:
                prefix_map[mod] = 1

        return count



class Solution1:
    def subarraysDivByK(self, nums: List[int], k: int) -> int:
        count = 0
        presum = 0
        premap = {0: 1}

        for n in nums:
            presum += n
            ressum = presum % k
            if ressum < 0:
                ressum += k
            if ressum in premap:
                count += premap[ressum]
                premap[ressum] += 1
            else:
                premap[ressum] = 1

        return count


def main():
    print('Hello World')

if __name__ == '__main__':
    main()