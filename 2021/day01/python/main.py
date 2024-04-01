def part1(input):
    increases = 0

    for i in range(1, len(input)):
        if input[i] > input[i - 1]:
            increases += 1

    return increases

def part2(input):
    increases = 0

    for i in range(0, len(input) - 3):
        if sum(input[i : i + 3]) < sum(input[i + 1 : i + 4]):
            increases += 1

    return increases

input = [int(line) for line in open('../input.txt').read().splitlines()]

print(part1(input))
print(part2(input))
