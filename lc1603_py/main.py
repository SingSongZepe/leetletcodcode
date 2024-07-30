
class ParkingSystem:

    def __init__(self, big: int, medium: int, small: int) -> None:
        self.left = [big, medium, small]

    def addCar(self, carType: int) -> bool:
        if self.left[carType-1] > 0:
            self.left[carType-1] -= 1
            return True
        else:
            return False

def main():
    print('Hello World')

if __name__ == '__main__':
    main()