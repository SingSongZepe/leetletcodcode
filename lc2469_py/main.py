
from typing import List
class Solution:
    def convertTemperature(self, celsius: float) -> List[float]:
        return [celsius + 273.15, celsius * 1.80 + 32.00]


def main():
    print('Hello World')

if __name__ == '__main__':
    main()