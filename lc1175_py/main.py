
class Solution:

    def numPrimeArrangements(self,n: int) -> int:
        def factorial(n: int) -> int:
            if n == 0:
                return 1
            else:
                return n * factorial(n - 1)

        primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
        pcount = 0
        for i in range(len(primes)):
            if primes[i] <= n:
                pcount += 1
            else:
                break
        return factorial(pcount) * factorial(n - pcount) % (10**9 + 7)

class Solution1:
    def numPrimeArrangements(self,n: int) -> int:
        def factorial(n: int) -> int:
            if n == 0:
                return 1
            else:
                return n * factorial(n - 1)

        def is_prime(num):
            if num < 2:
                return False
            for i in range(2, int(num ** 0.5) + 1):
                if num % i == 0:
                    return False
            return True

        pcount = 0
        for num in range(1, n + 1):
            if is_prime(num):
                pcount += 1

        return factorial(pcount) * factorial(n - pcount) % (10**9 + 7)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()