t = 0
s = []
a = 0
b = 0
br = False

f = open("data.txt", "r")

for l in f:
    for c in l:
        s.append(c)

        
    for i in range(len(s)):
        if s[int(i)].isnumeric():
            a = s[i]
            br = True
            break

    for i in range(len(s)):
        if s[(len(s) - 1) - i].isnumeric:
            b = s[(len(s) - 1) - i]
            br = True
            s = []
            break
    
    t = int(str(a) + str(b))

print(t)