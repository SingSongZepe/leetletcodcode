
class Solution:
    def angleClock(self, hour: int, minutes: int) -> float:
        result = abs(30 * hour - 11 / 2 * minutes)
        return result if result <= 180 else 360 - result


def main():
    print('Hello World')

if __name__ == '__main__':
    main()