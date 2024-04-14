def part1(input):
    for i in range(len(input)):
        for j in range(i + 1, len(input)):
            if input[i] + input[j] == 2020:
                return input[i] * input[j]

    raise Exception('no sum found')

def part2(input):
    for i in range(len(input)):
        for j in range(i + 1, len(input)):
            for k in range(j + 1, len(input)):
                if input[i] + input[j] + input[k] == 2020:
                    return input[i] * input[j] * input[k]

    raise Exception('no sum found')

input = [int(val) for val in open('../input.txt').read().splitlines()]

print(part1(input))
print(part2(input))
