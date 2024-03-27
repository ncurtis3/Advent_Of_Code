import re
import functools
num_re = re.compile("[0-9]+")

def pound(spring_str, nums) :
    # print("----------\nPound:")
    # print("Str: {}\nNums: {}".format(spring_str, nums))

    num = nums[0]

    if len(spring_str) < num :
        # print("str not big enough")
        return 0

    check_springs = spring_str[:num]

    if '.' in check_springs :
        # print("num doesn't fit")
        return 0

    if len(spring_str) == num :
        if len(nums) == 1 :
            # print("last possibility")
            return 1
        else :
            # print("Too many nums")
            return 0
        
    if spring_str[num] == '#' :
        # print("not alone")
        return 0
    
    return calc(spring_str[num + 1:], nums[1:])

def dot(spring_str, nums) :
    # print("----------Dot")
    return calc(spring_str[1:], nums)

@functools.lru_cache(maxsize=None)
def calc(spring_str, nums) :
    # print("----------\nCalc: ")
    # print("Str: {}\nNums: {}".format(spring_str, nums))

    if not nums :
        if not '#' in spring_str :
            # print("No more springs")
            return 1
        else :
            # print("Too many springs")
            return 0
        
    if not spring_str :
        # print("Too many nums")
        return 0
    
    next_char = spring_str[0]

    if next_char == '#' :
        return pound(spring_str, nums)
    elif next_char == '.' :
        return dot(spring_str, nums)
    else :
        return pound(spring_str, nums) + dot(spring_str, nums)
    
output = 0

for line in open("/home/nick/Advent_Of_Code/Hot_Springs.txt").readlines():
    spring_str, nums = line.split()

    num_list = [int(i) for i in nums.split(',')]
    mult_list = num_list
    mult_str = spring_str

    for i in range(0, 4) :
        mult_list = mult_list + num_list
        mult_str = mult_str + "?" + spring_str

    # print(mult_str)
    # print(mult_list)

    output += calc(mult_str, tuple(mult_list))

print(output)