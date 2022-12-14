f = open("data.txt", "r")
for x in f:
    pair = x.split(",")
    print(pair)
