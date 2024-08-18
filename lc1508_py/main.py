

from typing import List


from typing import List, Tuple

class Solution:
    MODULUS = 10**9 + 7

    def count_and_sum_subarrays(self, array: List[int], threshold: int) -> Tuple[int, int]:
        count, total_sum, current_window_sum, running_sum = 0, 0, 0, 0
        start = 0

        for end, num in enumerate(array):
            running_sum += num * (end - start + 1)
            current_window_sum += num
            while current_window_sum > threshold:
                running_sum -= current_window_sum
                current_window_sum -= array[start]
                start += 1
            count += end - start + 1
            total_sum += running_sum

        return count, total_sum

    def calculate_sum_of_first_k_subarrays(self, array: List[int], k: int) -> int:
        low, high = min(array), sum(array)
        while low < high:
            mid = (low + high) // 2
            if self.count_and_sum_subarrays(array, mid)[0] < k:
                low = mid + 1
            else:
                high = mid
        count, sum_value = self.count_and_sum_subarrays(array, low)
        return sum_value - low * (count - k)

    def rangeSum(self, nums: List[int], n: int, left: int, right: int) -> int:
        result = (self.calculate_sum_of_first_k_subarrays(nums, right) -
                  self.calculate_sum_of_first_k_subarrays(nums, left - 1))
        return result % self.MODULUS

class Solution:
    def rangeSum(self, nums: List[int], n: int, left: int, right: int) -> int:
        


        pass


def main():
    print('Hello World')

if __name__ == '__main__':
    main()