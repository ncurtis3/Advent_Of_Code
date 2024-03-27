raw_text = open('/home/nick/Advent_Of_Code/Pipe_Maze.txt', 'r').read()
i = 0
chars = list()
row_num = 0

s_pos = (0,0)

while i < len(raw_text) :
    chars.append(list())

    while i < len(raw_text) and raw_text[i] != '\n' :
        chars[row_num].append(raw_text[i])

        if raw_text[i] == 'S' :
            s_pos = (row_num, len(chars[row_num]) - 1)

        i += 1

    row_num += 1
    i += 1

process = (0,0)
processed = [s_pos]

if s_pos[0] != 0 and (chars[s_pos[0] - 1][s_pos[1]] == '|' or chars[s_pos[0] - 1][s_pos[1]] == '7' or chars[s_pos[0] - 1][s_pos[1]] == 'F') :
    process = (s_pos[0] - 1, s_pos[1])

elif s_pos[0] != len(chars) - 1 and (chars[s_pos[0] + 1][s_pos[1]] == '|' or chars[s_pos[0] + 1][s_pos[1]] == 'L' or chars[s_pos[0] + 1][s_pos[1]] == 'J') :
    process = (s_pos[0] + 1, s_pos[1])

elif s_pos[1] != 0 and (chars[s_pos[0]][s_pos[1] - 1] == '-' or chars[s_pos[0]][s_pos[1] - 1] == 'L' or chars[s_pos[0]][s_pos[1] - 1] == 'F') : 
    process = (s_pos[0], s_pos[1] - 1)

elif s_pos[1] != len(chars[s_pos[0]]) and (chars[s_pos[0]][s_pos[1] + 1] == '-' or chars[s_pos[0]][s_pos[1] + 1] == 'J' or chars[s_pos[0]][s_pos[1] + 1] == '7'):
    process = (s_pos[0], s_pos[1] + 1)

count = 0

while process != s_pos:
    processed.append(process)
    count += 1

    if chars[process[0]][process[1]] == '|' :
        if not (process[0] - 1, process[1]) in processed :
            process = (process[0] - 1, process[1])
        elif not (process[0] + 1, process[1]) in processed : 
            process = (process[0] + 1, process[1])
        else :
            process = s_pos

    elif chars[process[0]][process[1]] == '-' :
        if not (process[0], process[1] - 1) in processed :
            process = (process[0], process[1] - 1)
        elif not (process[0], process[1] + 1) in processed : 
            process = (process[0], process[1] + 1)
        else :
            process = s_pos

    elif chars[process[0]][process[1]] == '7' :
        if not (process[0], process[1] - 1) in processed :
            process = (process[0], process[1] - 1)
        elif not (process[0] + 1, process[1]) in processed : 
            process = (process[0] + 1, process[1])
        else :
            process = s_pos

    elif chars[process[0]][process[1]] == 'L' :
        if not (process[0], process[1] + 1) in processed :
            process = (process[0], process[1] + 1)
        elif not (process[0] - 1, process[1]) in processed : 
            process = (process[0] - 1, process[1])
        else :
            process = s_pos

    elif chars[process[0]][process[1]] == 'F' :
        if not (process[0], process[1] + 1) in processed :
            process = (process[0], process[1] + 1)
        elif not (process[0] + 1, process[1]) in processed : 
            process = (process[0] + 1, process[1])
        else :
            process = s_pos

    elif chars[process[0]][process[1]] == 'J' :
        if not (process[0], process[1] - 1) in processed :
            process = (process[0], process[1] - 1)
        elif not (process[0] - 1, process[1]) in processed : 
            process = (process[0] - 1, process[1])
        else :
            process = s_pos

print(processed)

sum = 0

for i in range(0, len(processed) - 1) :
    sum += (processed[i][0] + processed[i + 1][0]) * (processed[i][1] - processed[i + 1][1])

sum += (processed[len(processed) - 1][0] + processed[0][0]) * (processed[len(processed) - 1][1] - processed[0][1])

area = abs(sum) / 2

print(abs(area))
print(abs(sum))
print(count + 1)

interior = area + 1 - ((count + 1) / 2)

print(interior)
'''
print(s_pos)
print(chars[s_pos[0]][s_pos[1]])
print(chars[s_pos[0] - 1][s_pos[1]])
print(chars[s_pos[0] + 1][s_pos[1]])
print(chars[s_pos[0]][s_pos[1] - 1])
print(chars[s_pos[0]][s_pos[1] + 1])
print(process)

while len(process) != 0 : 
    count += 1
    process_num = len(process)
    print(process)

    for i in range(0, process_num) :
        position = process.pop(0)
        print(chars[position[0]][position[1]])

        if chars[position[0]][position[1]] == '|' :
            processed.append(position)

            if not (position[0] - 1, position[1]) in processed :
                process.append((position[0] - 1, position[1]))
            elif not (position[0] + 1, position[1]) in processed :
                process.append((position[0] + 1, position[1]))

        elif chars[position[0]][position[1]] == '-' :
            processed.append(position)

            if not (position[0], position[1] - 1) in processed :
                process.append((position[0], position[1] - 1))
            elif not (position[0], position[1] + 1) in processed :
                process.append((position[0], position[1] + 1))

        elif chars[position[0]][position[1]] == 'L' :
            processed.append(position)

            if not (position[0] - 1, position[1]) in processed :
                process.append((position[0] - 1, position[1]))
            elif not (position[0], position[1] + 1) in processed :
                process.append((position[0], position[1] + 1))

        elif chars[position[0]][position[1]] == 'J' :
            processed.append(position)

            if not (position[0] - 1, position[1]) in processed :
                process.append((position[0] - 1, position[1]))
            elif not (position[0], position[1] - 1) in processed :
                process.append((position[0], position[1] - 1))

        elif chars[position[0]][position[1]] == '7' :
            processed.append(position)

            if not (position[0], position[1] - 1) in processed :
                process.append((position[0], position[1] - 1))
            elif not (position[0] + 1, position[1]) in processed :
                process.append((position[0] + 1, position[1]))

        elif chars[position[0]][position[1]] == 'F' :
            processed.append(position)

            if not (position[0] + 1, position[1]) in processed :
                process.append((position[0] + 1, position[1]))
            elif not (position[0], position[1] + 1) in processed :
                process.append((position[0], position[1] + 1))

print(count)
'''