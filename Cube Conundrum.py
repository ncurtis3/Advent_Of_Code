import re

f = open("/home/nick/advent_of_code/2023/Cube Conundrum.txt", "r")
lines = f.readlines()
punctuation = re.compile(r";|:|,")
game_regex = re.compile("Game ([0-9]+)")
grab_regex = re.compile("([0-9]+) (green|red|blue)")
sum = 0
pow_sum = 0
max_red = 12
max_blue = 14
max_green = 13

for line in lines :
    split = punctuation.split(line)
    game_num = int(game_regex.search(split[0]).group(1))
    min_red = 0
    min_green = 0
    min_blue = 0

    i = 1

    pass_game = True

    while i < len(split):
        grab_match = grab_regex.search(split[i])
        num = int(grab_match.group(1))
        color = grab_match.group(2)

        if color == "red" :
            if num > max_red :
                pass_game = False

            if num > min_red :
                min_red = num
        elif color == "green" :
            if num > max_green :
                pass_game = False

            if num > min_green :
                min_green = num
        elif color == "blue" :
            if num > max_blue :
                pass_game = False

            if num > min_blue :
                min_blue = num
        
        i += 1

    if pass_game :
        sum += game_num

    pow_sum += min_red * min_green * min_blue

print(sum)
print(pow_sum)