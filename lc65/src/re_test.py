import re


def test1():
    test_cases = ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
    for test_case in test_cases:
        if re.match(r"^[-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?$", test_case):
            print(test_case, "is a valid number")
        else:
            print(test_case, "is not a valid number")
    print()

def test2():
    test_cases = ["abc", "1a", "12a", "123abc", "123 456", "123.456.789"]
    for test_case in test_cases:
        if re.match(r"^[-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?$", test_case):
            print(test_case, "is a valid number")
        else:
            print(test_case, "is not a valid number")
    print()

if __name__ == '__main__':
    test1()
    test2()

