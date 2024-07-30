
from typing import List
class Solution:
    def asteroidsDestroyed(self, mass: int, asteroids: List[int]) -> bool:
        # Sort the asteroids in descending order
        asteroids.sort()
        for m in asteroids:
            if mass >= m:
                mass += m
            else:
                return False
        return True


def main():
    print('Hello World')

if __name__ == '__main__':
    main()