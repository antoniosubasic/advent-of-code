int Part1(Claim[] input, out int[,] fabric)
{
    var max = Math.Max(input.Max(claim => claim.X + claim.Width), input.Max(claim => claim.Y + claim.Height));
    fabric = new int[max, max];

    foreach (var claim in input)
    {
        for (var x = claim.X; x < claim.X + claim.Width; x++)
        {
            for (var y = claim.Y; y < claim.Y + claim.Height; y++)
            {
                fabric[x, y]++;
            }
        }
    }

    return fabric.Cast<int>().Count(val => val > 1);
}

int Part2(Claim[] input, int[,] fabric)
{
    for (var x = 0; x < fabric.GetLength(0); x++)
    {
        for (var y = 0; y < fabric.GetLength(1); y++)
        {
            if (fabric[x, y] > 1)
            {
                for (var i = 0; i < input.Length; i++)
                {
                    if (x >= input[i].X && x < input[i].X + input[i].Width && y >= input[i].Y && y < input[i].Y + input[i].Height)
                    {
                        input[i].Overlaps = true;
                    }
                }
            }
        }
    }

    return input.First(claim => !claim.Overlaps).Id;
}


var input = File.ReadAllLines("../input.txt").Select(line =>
{
    var parts = line.Split(' ');
    var coords = parts[2].Split(',');
    var size = parts[3].Split('x');
    return new Claim(
        int.Parse(parts[0][1..]),
        int.Parse(coords[0]),
        int.Parse(coords[1][..^1]),
        int.Parse(size[0]),
        int.Parse(size[1])
    );
}).ToArray();

Console.WriteLine(Part1(input, out var fabric));
Console.WriteLine(Part2(input, fabric));


record Claim(int Id, int X, int Y, int Width, int Height)
{
    public bool Overlaps { get; set; }
}
