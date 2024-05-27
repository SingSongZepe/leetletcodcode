
class State:
    START = 1
    SIGN = 2
    SIGN_NUMBER = 3
    DOT_NUMBER = 4
    SIGN_DOT_NUMBER = 5
    EXP = 6
    SIGN_EXP = 7
    SIGN_EXP_NUMBER = 8
    DOT = 9
    def __init__(self) -> None:
        self.mode = State.START
    def change_mode(self, c: str) -> bool:
        if c.isdigit(): # \d
            if self.mode == State.START: # 13
                self.mode = State.SIGN_NUMBER
            elif self.mode == State.SIGN: # 23
                self.mode = State.SIGN_NUMBER
            elif self.mode == State.SIGN_NUMBER: # 33
                self.mode = State.SIGN_NUMBER
            elif self.mode == State.DOT_NUMBER: # 45
                self.mode = State.SIGN_DOT_NUMBER
            elif self.mode == State.SIGN_DOT_NUMBER: # 55
                self.mode = State.SIGN_DOT_NUMBER
            elif self.mode == State.EXP: # 68
                self.mode = State.SIGN_EXP_NUMBER
            elif self.mode == State.SIGN_EXP_NUMBER: # 88
                self.mode = State.SIGN_EXP_NUMBER
            elif self.mode == State.DOT: # 95
                self.mode = State.SIGN_DOT_NUMBER
            elif self.mode == State.SIGN_EXP: # 78
                self.mode = State.SIGN_EXP_NUMBER
            else:
                return False
        elif c == '.': # dot
            if self.mode == State.START: # 19
                self.mode = State.DOT
            elif self.mode == State.SIGN: # 29
                self.mode = State.DOT
            elif self.mode == State.SIGN_NUMBER: # 34
                self.mode = State.DOT_NUMBER
            else:
                return False
        elif c == 'e' or c == 'E': # e
            if self.mode == State.SIGN_DOT_NUMBER: # 56
                self.mode = State.EXP
            elif self.mode == State.SIGN_NUMBER: # 36
                self.mode = State.EXP
            elif self.mode == State.DOT_NUMBER: # 46
                self.mode = State.EXP
            else:
                return False
        elif c == '+' or c == '-': # + -
            if self.mode == State.START: # 12
                self.mode = State.SIGN
            elif self.mode == State.EXP: # 67
                self.mode = State.SIGN_EXP
            else:
                return False
        else:
            return False
        return True

class Solution:
    def isNumber(self, s: str) -> bool:
        state = State()
        for c in s:
            if not state.change_mode(c):
                return False
        if state.mode == State.SIGN_NUMBER or \
                state.mode == State.DOT_NUMBER or \
                state.mode == State.SIGN_DOT_NUMBER or \
                state.mode == State.SIGN_EXP_NUMBER:
            return True
        return False

class Solution1:
    def isNumber(self, s: str) -> bool:

        s = s.replace("E", "e")
        stack = s.split("e")
        if len(stack) > 2:
            return False

        def is_int(s):
            if not s:   # check if empty
                return False
            if s[0] not in "0123456789+-":  # check first char
                return False
            for i in range(len(s) - 1, 0, -1):  # check all other chars
                if s[i] not in "0123456789":
                    return False
            for char in s:  # at least has one digit in s
                if char in "01234567890":
                    return True
            return False

        def is_dec(s):
            if not s:   # check if empty
                return False
            if len(s.split(".")) != 2:  # check if there is one "."
                return False
            if s[0] not in "0123456789.+-": # check first char
                return False
            for i in range(len(s) - 1, 0, -1): # check all other chars
                if s[i] not in "0123456789.":
                    return False
            for char in s:      # at least has one digit in s
                if char in "0123456789":
                    return True
            return False

        if not is_dec(stack[0]) and not is_int(stack[0]):
            return False
        if len(stack) > 1 and not is_int(stack[1]):
            return False
        return True


def main():
    print('Hello World')

if __name__ == '__main__':
    main()