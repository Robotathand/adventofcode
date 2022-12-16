total = 0
maxtotal1 = 0
maxtotal2 = 0
maxtotal3 = 0

def add():
    global total
    global maxtotal1
    global maxtotal2
    global maxtotal3
    f = open("data.txt", "r")
    for x in f:
        if x != "\n":
            total = total + int(x)
        if x == "\n":
            if total > maxtotal1:
                maxtotal3 = maxtotal2
                maxtotal2 = maxtotal1
                maxtotal1 = total
            elif total > maxtotal2:
                maxtotal3 = maxtotal2
                maxtotal2 = total
            elif total > maxtotal3:
                maxtotal3 = total
            total = 0
#        print(maxtotal1, maxtotal2, maxtotal3)
#        print(maxtotal1 + maxtotal2 + maxtotal3)

add()
#print(maxtotal1, maxtotal2, maxtotal3)
print(maxtotal1 + maxtotal2 + maxtotal3)
