import re

f = open("/home/nick/advent_of_code/2023/trebuchet.txt", "r")
lines = f.readlines()
regex = re.compile(r"[0-9]|one|two|three|four|five|six|seven|eight|nine")
sum = 0

for line in lines :
    first_match = regex.search(line)
    last_match = regex.search(line)

    for i in range (1, len(line)) :
        if regex.search(line[i:]) != None :
            last_match = regex.search(line[i:])

    first = first_match.group(0)
    last = last_match.group(0)

    if first == "one" :
        first = "1"
    elif first == "two" :
        first = "2"
    elif first == "three" :
        first = "3"
    elif first == "four" :
        first = "4"
    elif first == "five" :
        first = "5"
    elif first == "six" :
        first = "6"
    elif first == "seven" :
        first = "7"
    elif first == "eight" :
        first = "8"
    elif first == "nine" :
        first = "9"

    if last == "one" :
        last = "1"
    elif last == "two" :
        last = "2"
    elif last == "three" :
        last = "3"
    elif last == "four" :
        last = "4"
    elif last == "five" :
        last = "5"
    elif last == "six" :
        last = "6"
    elif last == "seven" :
        last = "7"
    elif last == "eight" :
        last = "8"
    elif last == "nine" :
        last = "9"
    
    sum += int(first + last)

print(sum)