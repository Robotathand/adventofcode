total = 0
maxtotal = 0

def add():
    global total
    global maxtotal
    f = open("data.txt", "r")
    for x in f:
        if x != "\n":
            total = total + int(x)
        if x == "\n":
            if total > maxtotal:
                maxtotal = total
            total = 0

add()
print(maxtotal)
