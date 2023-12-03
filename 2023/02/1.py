s = []
br = False
t = 0
sc = ""

f = open("data.txt", "r")

for l in f:
    s = []
    s = l.split(" ")
    br = False
    for i in range (len(s)):
        if s[i] == "red," or s[i] == "red;" or s[i] == "red":
            if int(s[i - 1]) > 12:
                br = True
        elif s[i] == "green," or s[i] == "green;" or s[i] == "green":
            if int(s[i - 1]) > 13:
                br = True
        elif s[i] == "blue," or s[i] == "blue;" or s[i] == "blue":
            if int(s[i - 1]) > 14:
                br = True
        
        print(s[i], "   ", s[i-1], "    ", br)

    if br == False:
        sc = s[1]
        sc = sc.rstrip(sc[-1])
        t += int(sc)
        print(sc)

f.close()

print(t)