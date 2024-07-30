

class Solution:
    def num_rolls_to_target(self, n: int, k: int, target: int) -> int:
        def make(n, k, target, memo):
            if n == 1:
                return 1
            if (n, target) in memo:
                return memo[(n, target)]

            total = 0
            for i in range(1, min(k, target - n + 1) + 1):
                total += make(n - 1, k, target - i, memo)

            memo[(n, target)] = total
            return total

        memo = {}
        result = make(n, k, target, memo)
        return result % 1000000007


def main():
    n = 30
    k = 30
    target = 500
    solution = Solution()
    result = solution.num_rolls_to_target(n, k, target)
    print(result)

if __name__ == '__main__':
    main()