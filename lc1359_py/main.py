
class Solution:
    def countOrders(self, n: int) -> int:
        return self.countOrders(n-1) * n * (2*n-1) % 1000000007 if n > 1 else 1

def main():
    print('Hello World')

if __name__ == '__main__':
    main()