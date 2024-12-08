from itertools import combinations

with open("src/inputs/day08.txt") as f:
    matrix = [list(line) for line in f.read().strip().split("\n")]

n, m = len(matrix), len(matrix[0])

def is_inside(i, j):
    return 0 <= i < n and 0 <= j < m

antennas = {}
for i in range(n):
    for j in range(m):
        if matrix[i][j] != '.':
            if not matrix[i][j] in antennas:
                antennas[matrix[i][j]] = []
            antennas[matrix[i][j]].append((i, j))

part_a_antinodes = set()
for coords in antennas.values():
    for (x1, y1), (x2, y2) in combinations(coords, 2):
        dx, dy = x2 - x1, y2 - y1

        if is_inside(x1 - dx, y1 - dy):
            part_a_antinodes.add((x1 - dx, y1 - dy))

        if is_inside(x2 + dx, y2 + dy):
            part_a_antinodes.add((x2 + dx, y2 + dy))

part_b_antinodes = set()
for coords in antennas.values():
    for (x1, y1), (x2, y2) in combinations(coords, 2):
        dx, dy = x2 - x1, y2 - y1

        # Move in direction of x2, y2, from x1, y1
        x, y = x1, y1
        while is_inside(x, y):
            part_b_antinodes.add((x, y))
            x -= dx
            y -= dy

        # Move in direction of x1, y1, from x2, y2
        x, y = x2, y2
        while is_inside(x, y):
            part_b_antinodes.add((x, y))
            x += dx
            y += dy

print(len(part_a_antinodes), len(part_b_antinodes))
