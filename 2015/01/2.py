floor = 0
char = 0

f = open("./data.txt", "r")

for l in f:
    for c in l:
        if c == "(":
            floor += 1
        elif c == ")":
            floor -= 1
        
        char += 1
        
        if floor == -1:
            print(char)
            exit()

f.close()