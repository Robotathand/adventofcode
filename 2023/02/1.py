# answer from https://github.com/oliver-ni/advent-of-code/blob/master/py/2023/day02.py used to locate problem
# problem: wored at the end of the line are "word\n"


split_line = []
total = 0
sc = ""

file = open("./2023/02/data.txt", "r")


for line in file:

    split_line = line.split(" ")
    br = False

    for i in range (len(split_line)):
        token = split_line[i].strip()
        print(token)

        if token == "red," or split_line[i] == "red;" or split_line[i] == "red" or split_line[i] == "red\n":
            if int(split_line[i - 1]) > 12:
                br = True
                break


        elif split_line[i] == "green," or split_line[i] == "green;" or split_line[i] == "green" or split_line[i] == "green\n":
            if int(split_line[i - 1]) > 13:
                br = True
                break

        elif split_line[i] == "blue," or split_line[i] == "blue;" or split_line[i] == "blue" or split_line[i] == "blue\n":
            if int(split_line[i - 1]) > 14:
                br = True
                break


    if br == False:
        sc = split_line[1].strip()
        line_id = sc[:-1]
        total += int(line_id)


file.close()

print(total)