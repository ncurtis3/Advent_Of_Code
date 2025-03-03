import copy

def check_vert(graph) :
    i = 0

    while i < len(graph[0]) - 1 :
        # print()
        check = True
        j = 0

        while check and j < len(graph) :
            # print("Comparing [{}][{}] and [{}][{}]: {}, {}".format(j, i, j, i + 1, graph[j][i], graph[j][i + 1]))
            if graph[j][i] != graph[j][i + 1] :
                check = False

            j += 1
        
        if check :
            left = i - 1
            right = i + 2

            while left >= 0 and right < len(graph[0]) :
                # print()
                j = 0

                while check and j < len(graph) :
                    # print("Comparing [{}][{}] and [{}][{}]: {}, {}".format(j, left, j, right, graph[j][left], graph[j][right]))
                    if graph[j][left] != graph[j][right] :
                        check = False

                    j += 1

                if not check :
                    break

                left -= 1
                right += 1

            if check :
                return i + 1
        
        i += 1

    return -1

def check_horiz(graph) :
    i = 0

    while i < len(graph) - 1:
        # print()
        check = True
        j = 0

        while check and j < len(graph[i]) :
            # print("Comparing [{}][{}] and [{}][{}]: {}, {}".format(i, j, i + 1, j, graph[i][j], graph[i + 1][j]))
            if graph[i][j] != graph[i + 1][j] :
                check = False

            j += 1

        if check :
            left = i - 1
            right = i + 2

            while left >= 0 and right < len(graph) :
                # print()
                j = 0

                while check and j < len(graph[left]) :
                    # print("Comparing [{}][{}] and [{}][{}]: {}, {}".format(left, j, right, j, graph[left][j], graph[right][j]))
                    if graph[left][j] != graph[right][j] :
                        check = False

                    j += 1

                if not check :
                    break

                left -= 1
                right += 1

            if check :
                return i + 1
                
        i += 1
        
    return -1

def check_smudge(graph, old_line) :

    for i in range(0, len(graph)) :
        for j in range(0, len(graph[i])) :
            if graph[i][j] == '.' :
                graph[i] = graph[i][0:j] + '#' + graph[i][j + 1:]
                replace = '.'
            else :
                graph[i] = graph[i][0:j] + '.' + graph[i][j + 1:]
                replace = '#'

            check = check_horiz(graph)

            if check != -1 and check != old_line :
                return check
            
            check = check_vert(graph)

            if check != -1 and check != old_line :
                return check
            
            graph[i] = graph[i][0:j] + replace + graph[i][j + 1:]
            
    return -1

graphs = open("/home/nick/Advent_Of_Code/Mirrors.txt").read().split("\n\n")

sum = 0

for graph in graphs :
    graph = graph.split('\n')

    check = check_vert(graph)

    if check == -1 :
        check = check_smudge(graph, check_horiz(graph))
        sum += 100 * check
        print(check)
    else :
        check = check_smudge(graph, check)
        sum += check

        print(check)

print(sum)