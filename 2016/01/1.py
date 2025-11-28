file = open("data.txt", "r")
f = file.read()
steps = f.split(", ")
direction = 0  # 0123 nesw
location = [0, 0]
for step in steps:
    if step[0] == "R":
        direction += 1
        if direction > 3:
            direction = 0
    elif step[0] == "L":
        direction -= 1
        if direction < 0:
            direction = 3

    length = int(step[1:])
    if direction == 0:
        location[1] += length
    elif direction == 1:
        location[0] += length
    elif direction == 2:
        location[1] -= length
    elif direction == 3:
        location[0] -= length

if location[0] < 0:
    location[0] *= -1
if location[1] < 0:
    location[1] *= -1

print(location[0] + location[1])
