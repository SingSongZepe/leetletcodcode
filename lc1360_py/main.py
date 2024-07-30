
class Solution:
    def daysBetweenDates(self, date1: str, date2: str) -> int:
        date1 = date1.split('-')
        date2 = date2.split('-')
        y1 = int(date1[0])
        y2 = int(date2[0])
        m1 = int(date1[1])
        m2 = int(date2[1])
        d1 = int(date1[2])
        d2 = int(date2[2])

        day_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

        def days(y, m, d):
            ty = y - 1971
            leap_year = 0
            for i in range(1971, y):
                if i % 4 == 0 and (i % 100 != 0 or i % 400 == 0):
                    leap_year += 1
            td = sum(day_in_month[:m-1]) + d
            return ty * 365 + leap_year + td + (1 if m > 2 and y % 4 == 0 and (y % 100 != 0 or y % 400 == 0) else 0)

        return abs(days(y2, m2, d2) - days(y1, m1, d1))


def days(y, m, d):
    day_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    ty = y - 1971
    leap_year = 0
    for i in range(1971, y):
        if i % 4 == 0 and (i % 100 != 0 or i % 400 == 0):
            leap_year += 1
    td = sum(day_in_month[:m-1]) + d
    return ty * 365 + leap_year + td + (1 if m > 2 and y % 4 == 0 and (y % 100 != 0 or y % 400 == 0) else 0)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()