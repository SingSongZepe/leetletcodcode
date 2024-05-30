import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, mass, asteroids, expected=None):
        result = Solution().asteroidsDestroyed(mass, asteroids)
        print(result)

    def test1(self):
        mass = 10
        asteroids = [3, 9, 19, 5, 21]
        self.t(mass, asteroids)

    def test2(self):
        mass = 5
        asteroids = [4, 9, 23, 4]
        self.t(mass, asteroids)


if __name__ == '__main__':
    unittest.main()