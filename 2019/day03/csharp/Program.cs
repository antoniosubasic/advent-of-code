int Part1(Instruction[][] input, out (List<Coordinate> a, List<Coordinate> b) wires)
{
    wires = ([], []);

    for (var i = 0; i < input.Length; i++)
    {
        var cord = new Coordinate();

        foreach (var instruction in input[i])
        {
            for (int j = 0; j < instruction.Distance; j++)
            {
                switch (instruction.Direction)
                {
                    case Direction.Up: cord.Y++; break;
                    case Direction.Right: cord.X++; break;
                    case Direction.Down: cord.Y--; break;
                    case Direction.Left: cord.X--; break;
                }

                (i == 0 ? wires.a : wires.b).Add(cord);
            }

        }
    }

    return wires.a.Intersect(wires.b).Min(cord => cord.ManhattenDistance);
}

int Part2((List<Coordinate> a, List<Coordinate> b) wires)
{
    var intersections = wires.a.Intersect(wires.b);
    var minSteps = int.MaxValue;

    foreach (var intersection in intersections)
    {
        var stepsWireA = wires.a.Select((cord, index) => (cord, index)).First(item => item.cord == intersection).index + 1;
        var stepsWireB = wires.b.Select((cord, index) => (cord, index)).First(item => item.cord == intersection).index + 1;
        minSteps = Math.Min(minSteps, stepsWireA + stepsWireB);
    }

    return minSteps;
}


var input = File.ReadAllLines("../input.txt").Select(wire =>
    wire
    .Split(',')
    .Select(instruction => new Instruction(instruction[0] switch
    {
        'U' => Direction.Up,
        'R' => Direction.Right,
        'D' => Direction.Down,
        'L' => Direction.Left,
        _ => throw new Exception("invalid direction")
    }, int.Parse(instruction[1..])))
    .ToArray()
).ToArray();

Console.WriteLine(Part1(input, out var wires));
Console.WriteLine(Part2(wires));


enum Direction
{
    Up,
    Right,
    Down,
    Left
}

record struct Instruction(Direction Direction, int Distance);

record struct Coordinate(int X, int Y)
{
    public Coordinate() : this(0, 0) { }
    public readonly int ManhattenDistance => Math.Abs(X) + Math.Abs(Y);
}
