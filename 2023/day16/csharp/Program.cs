int Part1(char[][] input, (int x, int y)? startingPosition = null, Direction? startingDirection = null)
{
    var tiles = input.Select(line => line.Select(c => (character: c, directions: new List<Direction>())).ToArray()).ToArray();
    var beams = new List<Beam> { new(startingPosition?.x ?? 0, startingPosition?.y ?? 0, startingDirection ?? Direction.Right) };
    var energized = new HashSet<(int x, int y)>();

    while (beams.Count != 0)
    {
        for (var i = 0; i < beams.Count; i++)
        {
            var beam = beams[i];

            if (beam.X < 0 || beam.X >= tiles[0].Length || beam.Y < 0 || beam.Y >= tiles.Length || tiles[beam.Y][beam.X].directions.Contains(beam.Direction))
            {
                beams.RemoveAt(i);
                i--;
            }
            else
            {
                var tile = tiles[beam.Y][beam.X];

                energized.Add((beam.X, beam.Y));
                tile.directions.Add(beam.Direction);

                switch (tile.character)
                {
                    case '/':
                        beam.Direction = beam.Direction switch
                        {
                            Direction.Up => Direction.Right,
                            Direction.Right => Direction.Up,
                            Direction.Down => Direction.Left,
                            Direction.Left => Direction.Down,
                            _ => throw new Exception("invalid direction")
                        };
                        break;

                    case '\\':
                        beam.Direction = beam.Direction switch
                        {
                            Direction.Up => Direction.Left,
                            Direction.Left => Direction.Up,
                            Direction.Down => Direction.Right,
                            Direction.Right => Direction.Down,
                            _ => throw new Exception("invalid direction")
                        };
                        break;

                    case '|':
                        if (beam.Direction is Direction.Left or Direction.Right)
                        {
                            beam.Direction = Direction.Up;
                            beams.Add(new(beam.X, beam.Y + 1, Direction.Down));
                        }
                        break;

                    case '-':
                        if (beam.Direction is Direction.Up or Direction.Down)
                        {
                            beam.Direction = Direction.Left;
                            beams.Add(new(beam.X + 1, beam.Y, Direction.Right));
                        }
                        break;
                }

                switch (beam.Direction)
                {
                    case Direction.Up:
                        beam.Y--;
                        break;

                    case Direction.Right:
                        beam.X++;
                        break;

                    case Direction.Down:
                        beam.Y++;
                        break;

                    case Direction.Left:
                        beam.X--;
                        break;
                }
            }
        }
    }

    return energized.Count;
}

int Part2(char[][] input)
{
    var energized = new HashSet<int>();

    for (var x = 0; x < input[0].Length; x++)
    {
        energized.Add(Part1(input, (x, 0), Direction.Down));
        energized.Add(Part1(input, (x, input.Length - 1), Direction.Up));
    }

    for (var y = 0; y < input.Length; y++)
    {
        energized.Add(Part1(input, (0, y), Direction.Right));
        energized.Add(Part1(input, (input[0].Length - 1, y), Direction.Left));
    }

    return energized.Max();
}


var input = File.ReadAllLines("../input.txt").Select(line => line.ToCharArray()).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));


enum Direction
{
    Up,
    Right,
    Down,
    Left
}

class Beam(int x, int y, Direction direction)
{
    public int X { get; set; } = x;
    public int Y { get; set; } = y;
    public Direction Direction { get; set; } = direction;
}
