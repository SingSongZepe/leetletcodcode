import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, ops, objs, expected=None):
        # Your ParkingSystem object will be instantiated and called as such:
        obj = ParkingSystem(objs[0][0], objs[0][1], objs[0][2])
        result = []
        for i in range(1, len(ops)):
            result.append(obj.addCar(objs[i][0]))

        print(result)
        self.assertEqual(result, expected)


    def test1(self):
        ops = ["ParkingSystem", "addCar", "addCar", "addCar", "addCar"]
        objs = [[1, 1, 0], [1], [2], [3], [1]]
        expected = [True, True, False, False]
        self.t(ops, objs, expected)


if __name__ == '__main__':
    unittest.main()