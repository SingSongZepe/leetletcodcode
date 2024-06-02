
class Solution:
    def dayOfYear(self, date: str) -> int:
        year = int(date[:4])
        month = int(date[5:7])
        day = int(date[8:])

        days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

        total = 0
        for i in range(month-1):
            total += days_in_month[i]
        total += day

        if month > 2 and year % 4 == 0 and (year % 100!= 0 or year % 400 == 0):
            total += 1

        return total

class Solution1:
    def dayOfYear(self, date: str) -> int:
        date = date.split('-')
        year = int(date[0])
        month = int(date[1])
        day = int(date[2])

        days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

        total = 0
        for i in range(month-1):
            total += days_in_month[i]
        total += day

        if month > 2 and year % 4 == 0 and (year % 100!= 0 or year % 400 == 0):
            total += 1

        return total


def main():
    print('Hello World')

if __name__ == '__main__':
    main()