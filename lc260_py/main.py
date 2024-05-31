

from typing import List
class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        txor = 0
        for num in nums:
            txor ^= num

        first_xor = 0
        second_xor = 0
        bit_dif = 0
        while (txor >> bit_dif) & 1 == 0:
            bit_dif += 1
        for n in nums:
            if (n >> bit_dif) & 1 == 1:
                first_xor ^= n
            else:
                second_xor ^= n
        return [first_xor, second_xor]



def main():
    print('Hello World')

if __name__ == '__main__':
    main()