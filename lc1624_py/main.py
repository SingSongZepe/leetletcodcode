
class Solution:
    def maxLengthBetweenEqualCharacters(self, s: str) -> int:
        cargo = [-1] * 26
        m = -1
        for idx, c in enumerate(s):
            if cargo[ord(c) - 97] == -1:
                cargo[ord(c) - 97] = idx
            else:
                m = max(m, idx - cargo[ord(c) - 97] - 1)
        return m


def main():
    print('Hello World')

if __name__ == '__main__':
    main()