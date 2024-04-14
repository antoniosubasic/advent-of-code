def part1(input, seats):
    for instruction in input:
        row = col = 0

        for i in range(7):
            if instruction[i] == 'B':
                row += 2 ** (6 - i)

        for i in range(3):
            if instruction[i + 7] == 'R':
                col += 2 ** (2 - i)

        seats.add(row * 8 + col)
    return max(seats)

def part2(seats):
    for i in range(128 * 8):
        if i not in seats and i - 1 in seats and i + 1 in seats:
            return i
    
    raise Exception('seat not found')

input = open('../input.txt').read().splitlines()

seats = set()
print(part1(input, seats))
print(part2(seats))
