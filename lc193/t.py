import re


ss = ['987-123-4567', '123 456 7890', '(123) 456-7890', '0(001) 345-0000']
pattern = r'(\d{3}-)|(\(\d{3}\) )|(\d{3}-\d{4})'

for s in ss:
    if re.match(pattern, s):
        print(s)
