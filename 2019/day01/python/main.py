def part1(input):
    return sum(int(i) // 3 - 2 for i in input)

def part2(input):
    fuel = 0

    for i in [int(num) for num in input]:
        while (i := i // 3 - 2) > 0:
            fuel += i

    return fuel

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
