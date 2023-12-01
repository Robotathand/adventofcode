t = 0
a = 0
b = 0

f = open("data.txt", "r")

for l in f:
    for c in l:
        if c.isnumeric() == True:
            a = c
            for c in l:
                if c.isnumeric():
                    