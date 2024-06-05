class Solution:
    def strongPasswordCheckerII(self, password: str) -> bool:
        if len(password) < 8:
            return False

        upper = False
        lower = False
        digit = False
        special = False
        adjacent = True
        special_char = '!@#$%^&*()-+'
        for idx, char in enumerate(password):
            if char.isupper():
                upper = True
            elif char.islower():
                lower = True
            elif char.isdigit():
                digit = True
            elif char in special_char:
                special = True
            if idx > 0 and char == password[idx-1]:
                adjacent = False

        return upper and lower and digit and special and adjacent


def main():
    print('Hello World')

if __name__ == '__main__':
    main()