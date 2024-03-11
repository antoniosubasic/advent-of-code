int Part1((char direction, int blocks)[] input)
{
    var direction = 0;
    int x = 0, y = 0;

    foreach (var instruction in input)
    {
        direction = (direction + (instruction.direction == 'R' ? 1 : -1) + 4) % 4;

        switch (direction)
        {
            case 0:
                y -= instruction.blocks;
                break;
            case 1:
                x += instruction.blocks;
                break;
            case 2:
                y += instruction.blocks;
                break;
            case 3:
                x -= instruction.blocks;
                break;
        }
    }

    return Math.Abs(x) + Math.Abs(y);
}

int Part2((char direction, int blocks)[] input)
{
    var direction = 0;
    (int x, int y) position = (0, 0);
    var visited = new HashSet<(int x, int y)>();

    foreach (var instruction in input)
    {
        direction = (direction + (instruction.direction == 'R' ? 1 : -1) + 4) % 4;

        for (var i = 0; i < instruction.blocks; i++)
        {
            switch (direction)
            {
                case 0:
                    position.y--;
                    break;
                case 1:
                    position.x++;
                    break;
                case 2:
                    position.y++;
                    break;
                case 3:
                    position.x--;
                    break;
            }

            if (!visited.Add(position)) { return Math.Abs(position.x) + Math.Abs(position.y); }
        }
    }

    throw new Exception("No location visited twice");
}

var input = File.ReadAllText("../input.txt").Split(", ").Select(instruction => (instruction[0], int.Parse(instruction[1..]))).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
