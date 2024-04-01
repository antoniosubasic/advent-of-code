from functools import reduce

def get_horizontal_and_vertical_mirrors(terrain) -> tuple[set[int], set[int]]:
    vertical_parts = []
    horizontal_parts = []

    for i, line in enumerate(terrain):
        horizontal_before = terrain[:i + 1]
        horizontal_after = terrain[i + 1:]
        (horizontal_before, horizontal_after) = (horizontal_before, horizontal_after[:len(horizontal_before)]) if len(horizontal_before) <= len(horizontal_after) else (horizontal_before[-len(horizontal_after):], horizontal_after)
        horizontal_parts.append((horizontal_before, horizontal_after[::-1]))

        vertical_parts.append([])
        for j in range(len(line) - 1):
            vertical_before = line[:j + 1]
            vertical_after = line[j + 1:]
            (vertical_before, vertical_after) = (vertical_before, vertical_after[:len(vertical_before)]) if len(vertical_before) <= len(vertical_after) else (vertical_before[-len(vertical_after):], vertical_after)
            vertical_parts[i].append((vertical_before, vertical_after[::-1]))
    
    horizontal_mirrors = [i + 1 for (i, item) in enumerate(horizontal_parts) if item[0] == item[1]]
    vertical_mirrors = [[i + 1 for (i, item) in enumerate(part) if item[0] == item[1]] for part in vertical_parts]

    return (set(horizontal_mirrors), reduce(set.intersection, map(set, vertical_mirrors)))

def part1(input):
    total = 0

    for terrain in input:
        (horizontal_mirrors, vertical_mirrors) = get_horizontal_and_vertical_mirrors(terrain)
        if horizontal_mirrors: total += horizontal_mirrors.pop() * 100
        elif vertical_mirrors: total += vertical_mirrors.pop()

    return total

def part2(input):
    total = 0

    for terrain in input:
        (horizontal_mirrors, vertical_mirrors) = get_horizontal_and_vertical_mirrors(terrain)

        found_horizontal_mirrors = horizontal_mirrors.pop() if horizontal_mirrors else None
        found_vertical_mirrors = vertical_mirrors.pop() if vertical_mirrors else None
        found = False

        for i in range(len(terrain)):
            if found: break
            for j in range(len(terrain[i])):
                if found: break
                terrain[i] = f'{terrain[i][:j]}{"." if terrain[i][j] == "#" else "#"}{terrain[i][j + 1:]}'
                (horizontal_mirrors, vertical_mirrors) = get_horizontal_and_vertical_mirrors(terrain)
                terrain[i] = f'{terrain[i][:j]}{"." if terrain[i][j] == "#" else "#"}{terrain[i][j + 1:]}'
                horizontal_mirrors.discard(found_horizontal_mirrors)
                vertical_mirrors.discard(found_vertical_mirrors)
                if len(horizontal_mirrors) != 0:
                    total += horizontal_mirrors.pop() * 100
                    found = True
                elif len(vertical_mirrors) != 0:
                    total += vertical_mirrors.pop()
                    found = True

    return total

input = [[line for line in terrain.split('\n') if line != ''] for terrain in open('../input.txt').read().split('\n\n')]

print(part1(input))
print(part2(input))
