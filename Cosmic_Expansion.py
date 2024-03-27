
def only_space_row(i) :
    found = False

    i = 0
    while i < len(line) and not found :
        if line[i] == '#' :
            found = True
        i += 1

    return found

raw_text = open('/home/nick/Advent_Of_Code/Cosmic_Expansion.txt', 'r').read()
i = 0
chars = list()
row_num = 0

while i < len(raw_text) :
    chars.append(list())

    while i < len(raw_text) and raw_text[i] != '\n' :
        chars[row_num].append(raw_text[i])
        i += 1

    row_num += 1
    i += 1

i = 0
empty_rows = list()

while i < len(chars) :
    found = False

    j = 0
    while j < len(chars[i]) and not found :
        if chars[i][j] == '#' :
            found = True
        j += 1

    if not found :
        empty_rows.append(i)

    i += 1

i = 0
empty_cols = list()

while i < len(chars[0]) :
    found = False

    j = 0
    while j < len(chars) and not found :
        if chars[j][i] == '#' :
            found = True
        j += 1

    if not found :
        empty_cols.append(i)

    i += 1

galaxies = list()

for i in range(0, len(chars)) :
    for j in range(0, len(chars[i])) :
        if chars[i][j] == '#' :
            count = 0

            while count < len(empty_rows) and empty_rows[count] < i :
                count += 1

            row_offset = count * 999999

            count = 0

            while count < len(empty_cols) and empty_cols[count] < j :
                count += 1

            col_offset = count * 999999

            galaxies.append((i + row_offset, j + col_offset))


sum = 0

for i in range(0, len(galaxies) - 1) :
    for j in range(i + 1, len(galaxies)) :
        sum += abs(galaxies[i][0] - galaxies[j][0]) + abs(galaxies[i][1] - galaxies[j][1])

print(sum)