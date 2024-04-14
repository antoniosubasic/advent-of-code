def part1(input):
    trees = 0
    for i, line in enumerate(input):
        if line[(i * 3) % len(line)] == '#':
            trees += 1
    return trees

def part2(input):
    trees = 1
    for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        count = 0
        for i, line in enumerate(input):
            if i % down == 0 and line[(i // down * right) % len(line)] == '#':
                count += 1
        trees *= count
    return trees

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
