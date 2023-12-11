import re

f = open("/Users/nicholascurtis/Advent of Code/2023/Gear Ratios.txt", "r")
num_regex = re.compile("[0-9]+")

lines = f.readlines()
sum = 0
array = []
map = {}

for line in lines :
    if line[len(line) - 1] == '\n' :
        array.append(line[:len(line) - 1])
    else :
        array.append(line)

for i in range(0, len(array)) :
    counter = 0
    num_match = num_regex.search(array[i])

    while num_match :
        '''
        print()
        print(num_match.group(0))
        print(array[i])
        print(len(array[i]))
        '''
        
        check_top = True
        check_bottom = True
        start = 0
        end = 0

        found = False

        if num_match.start(0) + counter == 0 :
            start = 0
        else :
            start = num_match.start(0) - 1 + counter

        if num_match.end(0) + counter == len(array[i]) :
            end = len(array[i])
        else :
            end = num_match.end(0) + 1 + counter

        if i == 0 :
            check_top = False

        if i == len(array) - 1 :
            check_bottom = False

        #print(start)
        #print(end)

        for check in range(start, end) :
            #print(check)

            if check_top :
                #print("Checking array[{}][{}]".format(i - 1, check))
                #print(symbol_regex.match(array[i - 1][check]))
                #print(array[i - 1][check])
                unicode = ord(array[i - 1][check])
                if unicode != 46 and ((unicode < 48 or unicode > 57)) :
                    found = True
                    break
            
            if check_bottom :
                #print("Checking array[{0}][{1}]".format(i + 1, check))
                #print(array[i + 1][check])
                unicode = ord(array[i + 1][check])
                if unicode != 46 and ((unicode < 48 or unicode > 57)) :
                    found = True
                    break

            if check == start or check == end - 1 :
                #print("Checking array[{}][{}]".format(i, check))
                #print(symbol_regex.match(array[i][check]))
                #print(array[i][check])
                unicode = ord(array[i][check])
                if unicode != 46 and ((unicode < 48 or unicode > 57)) :
                    found = True
                    break

        if found :
            #print("Adding {}".format(num_match.group(0)))
            sum += int(num_match.group(0))

        counter += num_match.end(0)
        num_match = num_regex.search(array[i][counter:])

print(sum)

def add_to_dict(i, j, num, map) :
    coord = "(" + str(i) + ", " + str(j) + ")"

    if coord in map :
        (count, mult) = map[coord]
        #print("Multiplying {0} by {1}".format(coord, count))
        map[coord] = (count + 1, mult * num)
    else :
        #print("Adding {0} at {1}".format(num, coord))
        map[coord] = (1, num)

sum = 0

for i in range(0, len(array)) :
    counter = 0
    num_match = num_regex.search(array[i])

    while num_match :
        '''
        print()
        print(num_match.group(0))
        print(array[i])
        print(len(array[i]))
        '''
        check_top = True
        check_bottom = True
        start = 0
        end = 0

        found = False

        if num_match.start(0) + counter == 0 :
            start = 0
        else :
            start = num_match.start(0) - 1 + counter

        if num_match.end(0) + counter == len(array[i]) :
            end = len(array[i])
        else :
            end = num_match.end(0) + 1 + counter

        if i == 0 :
            check_top = False

        if i == len(array) - 1 :
            check_bottom = False

        #print(start)
        #print(end)

        for check in range(start, end) :
            #print(check)

            if check_top :
                #print("Checking array[{}][{}]".format(i - 1, check))
                #print(symbol_regex.match(array[i - 1][check]))
                #print(array[i - 1][check])
                unicode = ord(array[i - 1][check])
                if ord(array[i - 1][check]) == 42 :
                    add_to_dict(i - 1, check, int(num_match.group(0)), map)
            
            if check_bottom :
                #print("Checking array[{0}][{1}]".format(i + 1, check))
                #print(array[i + 1][check])
                if ord(array[i + 1][check]) == 42 :
                    add_to_dict(i + 1, check, int(num_match.group(0)), map)

            if check == start or check == end - 1 :
                #print("Checking array[{}][{}]".format(i, check))
                #print(symbol_regex.match(array[i][check]))
                #print(array[i][check])
                if ord(array[i][check]) == 42 :
                    add_to_dict(i, check, int(num_match.group(0)), map)

        if found :
            #print("Adding {}".format(num_match.group(0)))
            sum += int(num_match.group(0))

        counter += num_match.end(0)
        num_match = num_regex.search(array[i][counter:])

for key in map.keys() :
    (num, mult) = map[key]

    if num == 2 :
        #print("Adding {} to sum".format(mult))
        sum += mult
    #else :
        #print("Ignoring {}".format(mult))

print(sum)