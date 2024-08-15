var input = File.ReadAllLines("../input.txt").Select(line => new Line(line)).ToArray();

var simpleDiagram = new Dictionary<(int x, int y), int>();
var advancedDiagram = new Dictionary<(int x, int y), int>();

foreach (var line in input)
{
    foreach (var coordinate in line.GetCoordinates())
    {
        if (line.Start.x == line.End.x || line.Start.y == line.End.y) { simpleDiagram[coordinate] = simpleDiagram.GetValueOrDefault(coordinate) + 1; }
        advancedDiagram[coordinate] = advancedDiagram.GetValueOrDefault(coordinate) + 1;
    }
}

Console.WriteLine(simpleDiagram.Count(field => field.Value >= 2));
Console.WriteLine(advancedDiagram.Count(field => field.Value >= 2));

readonly struct Line
{
    public (int x, int y) Start { get; }
    public (int x, int y) End { get; }

    public Line(string line)
    {
        var parts = line.Split(" -> ");
        var start = parts[0].Split(',');
        var end = parts[1].Split(',');

        Start = (int.Parse(start[0]), int.Parse(start[1]));
        End = (int.Parse(end[0]), int.Parse(end[1]));
    }

    public IEnumerable<(int x, int y)> GetCoordinates()
    {
        var x1 = Start.x;
        var y1 = Start.y;
        var x2 = End.x;
        var y2 = End.y;

        var dx = Math.Abs(x2 - x1);
        var dy = Math.Abs(y2 - y1);
        var sx = x1 < x2 ? 1 : -1;
        var sy = y1 < y2 ? 1 : -1;
        var err = dx - dy;

        while (true)
        {
            yield return (x1, y1);

            if (x1 == x2 && y1 == y2) { break; }
            else
            {
                var e2 = 2 * err;

                if (e2 > -dy)
                {
                    err -= dy;
                    x1 += sx;
                }

                if (e2 < dx)
                {
                    err += dx;
                    y1 += sy;
                }
            }
        }
    }
}
