#!/usr/bin/python3
with open("input01.txt") as inputfilehmmyes:
    directions = [line.strip() for line in inputfilehmmyes]

direction_pairs = []
for direction in directions:
    direction_pairs.append([direction.split(' ')[0], direction.split(' ')[1]])

horizontal_pos = 0
vertical_pos = 0
for movement in direction_pairs:
    if movement[0] == 'forward':
        horizontal_pos += int(movement[1])
    elif movement[0] == 'down':
        vertical_pos += int(movement[1])
    elif movement[0] == 'up':
        vertical_pos -= int(movement[1])

print("Horizontal position: {}\nVertical position: {}".format(horizontal_pos, vertical_pos))
print("Answer:", horizontal_pos*vertical_pos)
