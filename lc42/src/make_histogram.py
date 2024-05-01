import matplotlib.pyplot as plt
import unittest
import matplotlib.pyplot as plt

def save(filename):
    plt.savefig(filename)

def draw(data):
    max_value = max(data) if data else 1.0

    bars = [(i, count) for i, count in enumerate(data)]
    x = [bar[0] for bar in bars]
    y = [bar[1] / max_value for bar in bars]

    plt.bar(x, y, color='b', alpha=0.5)

    for i, v in enumerate(y):
        plt.text(i, v + 0.01, str(data[i]), ha='center')

    plt.xlabel('X')
    plt.ylabel('Y')
    plt.title('Histogram')
    plt.xticks(x)
    plt.yticks([0, 0.2, 0.4, 0.6, 0.8, 1.0])
    plt.grid(axis='y', linestyle='--', alpha=0.7)

    plt.show()

class Test(unittest.TestCase):
    def test1(self):
        data = [5, 4, 1, 2]
        draw(data)

    def test2(self):
        data = [0,1,0,2,1,0,1,3,2,1,2,1]
        draw(data)

    def test3(self):
        data = [5,5,4,7,8,2,6,9,4,5]
        draw(data)
        save('test3.png')

if __name__ == '__main__':
    unittest.main()

