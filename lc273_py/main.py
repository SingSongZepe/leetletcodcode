
class Solution:
    def numberToWords(self, num: int) -> str:
        d = {
                0: "Zero",
                1: "One",
                2: "Two",
                3: "Three",
                4: "Four",
                5: "Five",
                6: "Six",
                7: "Seven",
                8: "Eight",
                9: "Nine",
                10: "Ten",
                11: "Eleven",
                12: "Twelve",
                13: "Thirteen",
                14: "Fourteen",
                15: "Fifteen",
                16: "Sixteen",
                17: "Seventeen",
                18: "Eighteen",
                19: "Nineteen",
                20: "Twenty",
                30: "Thirty",
                40: "Forty",
                50: "Fifty",
                60: "Sixty",
                70: "Seventy",
                80: "Eighty",
                90: "Ninety",
                100: "Hundred",
                1000: "Thousand",
                1000000: "Million",
                1000000000: "Billion"
            }

        def lessThanThousand(num):
            if num == 0:
                return ""
            if num <= 20:
                return d[num]
            elif num < 100:
                return d[num // 10 * 10] + (" " + d[num % 10] if num % 10 != 0 else "")
            else:
                l = lessThanThousand(num % 100)
                return d[num // 100] + " " + d[100] + ( " " + l if l != "" else "")

        if num == 0:
            return d[0]
        elif num < 1000:
            return lessThanThousand(num)
        else:
            words = ""
            
            seg1 = num % 1000
            seg2 = (num // 1000) % 1000
            seg3 = (num // 1000000) % 1000
            seg4 = (num // 1000000000) % 1000
            
            if seg4 > 0:
                words += lessThanThousand(seg4) + " " + d[1000000000] + " "
            if seg3 > 0:
                words += lessThanThousand(seg3) + " " + d[1000000] + " "
            if seg2 > 0:
                words += lessThanThousand(seg2) + " " + d[1000] + " "
            if seg1 > 0:
                words += lessThanThousand(seg1) + " "
            
            return words[:-1]



class Solution1:
    def numberToWords(self, num: int) -> str:
        lessThantwenty = ['', 'One', 'Two', 'Three', 'Four', 'Five', 'Six', 'Seven', 'Eight', 'Nine', 'Ten', 'Eleven', 'Twelve', 'Thirteen', 'Fourteen', 'Fifteen', 'Sixteen', 'Seventeen', 'Eighteen', 'Nineteen']
        tens = ['', '', 'Twenty', 'Thirty', 'Forty', 'Fifty', 'Sixty', 'Seventy', 'Eighty', 'Ninety']
        thousand = ['', 'Thousand', 'Million', 'Billion']
        
        def lessThanThousand(num):
            if num == 0:
                return ''
            elif num < 20:
                return lessThantwenty[num]
            elif num < 100:
                return tens[num // 10] + (' ' + lessThantwenty[num % 10] if num % 10 != 0 else '')
            else:
                return lessThantwenty[num // 100] + ' Hundred' + (' ' + lessThanThousand(num % 100) if num % 100 != 0 else '') 
        
        if num == 0:
            return 'Zero'
        elif num < 1000:
            return lessThanThousand(num)
        else:
            words = ''
            for i in range(4):
                seg = num % 1000
                if seg > 0:
                    words = lessThanThousand(seg) + ' ' + thousand[i] + ' ' + words
                num //= 1000
            return words.strip()



class Solution2:
    def numberToWords(self, num: int) -> str:
        
        if num == 0:
            return 'Zero'
        
        lessThantwenty = ['', 'One', 'Two', 'Three', 'Four', 'Five', 'Six', 'Seven', 'Eight', 'Nine', 'Ten', 'Eleven', 'Twelve', 'Thirteen', 'Fourteen', 'Fifteen', 'Sixteen', 'Seventeen', 'Eighteen', 'Nineteen']
        tens = ['', '', 'Twenty', 'Thirty', 'Forty', 'Fifty', 'Sixty', 'Seventy', 'Eighty', 'Ninety']

        def recv(num) -> str:
            if num == 0:
                res = ''
            elif num < 20:
                res = lessThantwenty[num]
            elif num < 100:
                res = tens[num // 10] + ' ' + lessThantwenty[num % 10]
            elif num < 1000:
                res = lessThantwenty[num // 100] + ' Hundred ' + recv(num % 100)
            elif num < 1000000:
                res = recv(num // 1000) + ' Thousand ' + recv(num % 1000)
            elif num < 1000000000:
                res = recv(num // 1000000) + ' Million ' + recv(num % 1000000)
            else:
                res = recv(num // 1000000000) + ' Billion ' + recv(num % 1000000000)
            
            return res.strip()
        
        return recv(num)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()