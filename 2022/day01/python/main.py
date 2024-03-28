def part1(input):
    return max(input)

def part2(input):
    return sum(sorted(input, reverse=True)[:3])

input = [sum([int(calorie) for calorie in entry.split('\n')]) for entry in open('../input.txt').read().split('\n\n')]

print(part1(input))
print(part2(input))
