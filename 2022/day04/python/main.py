def part1(input):
    return sum([1 for pair in input if (pair[0][0] >= pair[1][0] and pair[0][1] <= pair[1][1]) or (pair[1][0] >= pair[0][0] and pair[1][1] <= pair[0][1])])

def part2(input):
    return sum([1 for pair in input if (pair[0][0] <= pair[1][0] and pair[0][1] >= pair[1][0]) or (pair[1][0] <= pair[0][0] and pair[1][1] >= pair[0][0])])

input = [[[int(val) for val in range.split('-')] for range in line.split(',')] for line in open('../input.txt').read().splitlines()]

print(part1(input))
print(part2(input))
