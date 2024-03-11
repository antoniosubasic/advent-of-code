int Part1(Tile[][] input, Coordinate start, out Node root)
{
    var visited = new HashSet<Coordinate>() { start };

    root = new Node(input[start.Y][start.X]);
    var current = root;

    while (true)
    {
        Coordinate? top = current.Tile.Coordinate.Y == 0 || !current.Tile.CanGoNorth ? null : new(current.Tile.Coordinate.X, current.Tile.Coordinate.Y - 1);
        Coordinate? right = current.Tile.Coordinate.X == input[0].Length - 1 || !current.Tile.CanGoEast ? null : new(current.Tile.Coordinate.X + 1, current.Tile.Coordinate.Y);
        Coordinate? bottom = current.Tile.Coordinate.Y == input.Length - 1 || !current.Tile.CanGoSouth ? null : new(current.Tile.Coordinate.X, current.Tile.Coordinate.Y + 1);
        Coordinate? left = current.Tile.Coordinate.X == 0 || !current.Tile.CanGoWest ? null : new(current.Tile.Coordinate.X - 1, current.Tile.Coordinate.Y);

        if ((top == start || right == start || bottom == start || left == start) && visited.Count > 2) { return current.Distance / 2 + 1; }
        else
        {
            if (top.HasValue && !visited.Contains(top.Value) && (input[top.Value.Y][top.Value.X].Type is TileType.NorthSouth or TileType.SouthEast or TileType.SouthWest))
            {
                visited.Add(top.Value);
                current.Next = new(input[top.Value.Y][top.Value.X], current);
            }
            else if (right.HasValue && !visited.Contains(right.Value) && (input[right.Value.Y][right.Value.X].Type is TileType.EastWest or TileType.NorthWest or TileType.SouthWest))
            {
                visited.Add(right.Value);
                current.Next = new(input[right.Value.Y][right.Value.X], current);
            }
            else if (bottom.HasValue && !visited.Contains(bottom.Value) && (input[bottom.Value.Y][bottom.Value.X].Type is TileType.NorthSouth or TileType.NorthEast or TileType.NorthWest))
            {
                visited.Add(bottom.Value);
                current.Next = new(input[bottom.Value.Y][bottom.Value.X], current);
            }
            else if (left.HasValue && !visited.Contains(left.Value) && (input[left.Value.Y][left.Value.X].Type is TileType.EastWest or TileType.NorthEast or TileType.SouthEast))
            {
                visited.Add(left.Value);
                current.Next = new(input[left.Value.Y][left.Value.X], current);
            }
            else { current = current.Parent ?? throw new Exception("parent not found"); }

            current = current.Next ?? throw new Exception("next element not found");
        }
    }
}

int Part2(Tile[][] input, Node root)
{
    var loop = new HashSet<Coordinate>();
    for (var node = root; node is not null; node = node.Next) { loop.Add(node.Tile.Coordinate); }

    var enclosed = 0;

    for (var y = 0; y < input.Length; y++)
    {
        for (var x = 0; x < input[y].Length; x++)
        {
            if (!loop.Contains(input[y][x].Coordinate))
            {
                var rays = new[]
                {
                    input[..y].Select(row => row[x]),
                    input[y][x..],
                    input[y..].Select(row => row[x]),
                    input[y][..x],
                }.Select(ray => ray.Where(tile => loop.Contains(tile.Coordinate)).ToArray());

                if (
                    new[]
                    {
                        rays.ElementAt(0).GetCrossingsVertically(),
                        rays.ElementAt(1).GetCrossingsHorizontally(),
                        rays.ElementAt(2).GetCrossingsVertically(),
                        rays.ElementAt(3).GetCrossingsHorizontally(),
                    }.All(crossings => crossings % 2 == 1)
                )
                {
                    enclosed++;
                }
            }
        }
    }

    return enclosed;
}


Coordinate? start = null;
var input = File.ReadAllLines("../input.txt").Select((line, y) => line.ToCharArray()
    .Select((tile, x) =>
    {
        var coordinate = new Coordinate(x, y);
        if (tile == 'S') { start = coordinate; }
        return new Tile(tile switch
        {
            '|' => TileType.NorthSouth,
            '-' => TileType.EastWest,
            'L' => TileType.NorthEast,
            'J' => TileType.NorthWest,
            '7' => TileType.SouthWest,
            'F' => TileType.SouthEast,
            '.' => TileType.None,
            'S' => TileType.Start,
            _ => throw new Exception($"unknown tile ({tile}) at {x}, {y}")
        }, coordinate);
    }).ToArray()
).ToArray();

Console.WriteLine(Part1(input, start ?? throw new Exception("start not found"), out var root));
Console.WriteLine(Part2(input, root));


public class Node(Tile tile, Node? parent = null)
{
    public Tile Tile { get; } = tile;
    public Node? Parent { get; } = parent;
    public Node? Next { get; set; }
    public int Distance { get; set; } = parent is null ? 0 : parent.Distance + 1;
}

public readonly record struct Tile(TileType Type, Coordinate Coordinate)
{
    public bool CanGoNorth => Type is TileType.NorthSouth or TileType.NorthEast or TileType.NorthWest or TileType.Start;
    public bool CanGoEast => Type is TileType.EastWest or TileType.NorthEast or TileType.SouthEast or TileType.Start;
    public bool CanGoSouth => Type is TileType.NorthSouth or TileType.SouthEast or TileType.SouthWest or TileType.Start;
    public bool CanGoWest => Type is TileType.EastWest or TileType.NorthWest or TileType.SouthWest or TileType.Start;
}

public static class Extensions
{
    public static int GetCrossingsVertically(this IEnumerable<Tile> ray)
    {
        var crossings = 0;
        var eastCrossings = 0;
        var westCrossings = 0;

        foreach (var tile in ray)
        {
            switch (tile.Type)
            {
                case TileType.EastWest:
                    crossings++;
                    break;

                case TileType.NorthEast:
                case TileType.SouthEast:
                    eastCrossings++;
                    break;

                case TileType.NorthWest:
                case TileType.SouthWest:
                    westCrossings++;
                    break;
            }
        }

        return crossings + Math.Min(eastCrossings, westCrossings);
    }

    public static int GetCrossingsHorizontally(this IEnumerable<Tile> ray)
    {
        var crossings = 0;
        var northCrossings = 0;
        var southCrossings = 0;

        foreach (var tile in ray)
        {
            switch (tile.Type)
            {
                case TileType.NorthSouth:
                    crossings++;
                    break;

                case TileType.NorthEast:
                case TileType.NorthWest:
                    northCrossings++;
                    break;

                case TileType.SouthEast:
                case TileType.SouthWest:
                    southCrossings++;
                    break;
            }
        }

        return crossings + Math.Min(northCrossings, southCrossings);
    }
}

public record struct Coordinate(int X, int Y);

public enum TileType
{
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    None,
    Start,
}
