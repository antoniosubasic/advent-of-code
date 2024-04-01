def part1(input):
    horizontal = depth = 0

    for line in input:
        (instruction, value) = line.split(' ')
        value = int(value)
        if instruction == 'forward': horizontal += value
        elif instruction == 'down': depth += value
        elif instruction == 'up': depth -= value

    return horizontal * depth

def part2(input):
    horizontal = depth = aim = 0

    for line in input:
        (instruction, value) = line.split(' ')
        value = int(value)
        if instruction == 'forward':
            horizontal += value
            depth += aim * value
        elif instruction == 'down': aim += value
        elif instruction == 'up': aim -= value

    return horizontal * depth

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
