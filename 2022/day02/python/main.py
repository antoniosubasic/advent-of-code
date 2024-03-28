def part1(input):
    sum = 0

    for game in input:
        (opponent, me) = game
        sum += 3 if chr(ord(opponent) + ord('X') - ord('A')) == me else (0 if (opponent == 'A' and me == 'Z') or (opponent == 'C' and me == 'Y') or (opponent == 'B' and me == 'X') else 6)
        sum += ord(me) - ord('X') + 1

    return sum

def part2(input):
    for i in range(len(input)):
        (opponent, todo) = input[i]
        
        if todo == 'X':
            input[i][1] = 'Z' if opponent == 'A' else ('X' if opponent == 'B' else 'Y')
        elif todo == 'Y':
            input[i][1] = chr(ord(opponent) + ord('X') - ord('A'))
        elif todo == 'Z':
            input[i][1] = 'Y' if opponent == 'A' else ('Z' if opponent == 'B' else 'X')

    return part1(input)


input = [line.split(' ') for line in open('../input.txt').read().splitlines()]

print(part1(input))
print(part2(input))
