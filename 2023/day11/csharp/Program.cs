long Part1And2(Galaxy input, int expansionMultiplier)
{
    input.ExpandGalaxy(expansionMultiplier);

    long sumOfLengths = 0;

    for (var y = 0; y < input.Galaxies.Count; y++)
    {
        var from = input.GetNthGalaxy(y);

        for (var x = y + 1; x < input.Galaxies.Count; x++)
        {
            var to = input.GetNthGalaxy(x);
            sumOfLengths += Math.Abs(from.X - to.X) + Math.Abs(from.Y - to.Y);
        }
    }

    return sumOfLengths;
}


var input = File.ReadAllLines("../input.txt");

Console.WriteLine(Part1And2(new Galaxy(input), 2));
Console.WriteLine(Part1And2(new Galaxy(input), 1_000_000));


class Galaxy
{
    public HashSet<Coordinate> Galaxies { get; private set; }
    public HashSet<int> EmptyColumns { get; }
    public HashSet<int> EmptyRows { get; }

    public Galaxy(string[] rawImage)
    {
        var image = rawImage.Select(row => row.ToCharArray()).ToArray();

        Galaxies = [];
        EmptyColumns = [];
        EmptyRows = [];

        for (var y = 0; y < image.Length; y++)
        {
            if (image[y].All(c => c == '.')) { EmptyRows.Add(y); }

            for (var x = 0; x < image[y].Length; x++)
            {
                if (image[y][x] == '#') { Galaxies.Add(new(x, y)); }
            }
        }

        for (var x = 0; x < image[0].Length; x++)
        {
            if (image.All(row => row[x] == '.')) { EmptyColumns.Add(x); }
        }
    }

    public void ExpandGalaxy(int factor)
    {
        for (var y = 0; y < EmptyRows.Count; y++)
        {
            var index = EmptyRows.ElementAt(y) + (factor - 1) * y;

            for (var i = 0; i < Galaxies.Count; i++)
            {
                if (Galaxies.ElementAt(i).Y > index) { Galaxies.ElementAt(i).Y += factor - 1; }
            }
        }

        for (var x = 0; x < EmptyColumns.Count; x++)
        {
            var index = EmptyColumns.ElementAt(x) + (factor - 1) * x;

            for (var i = 0; i < Galaxies.Count; i++)
            {
                if (Galaxies.ElementAt(i).X > index) { Galaxies.ElementAt(i).X += factor - 1; }
            }
        }
    }

    public Coordinate GetNthGalaxy(int nth) => Galaxies.OrderBy(galaxy => galaxy.Y).ThenBy(galaxy => galaxy.X).ElementAt(nth);
}

class Coordinate(int x, int y)
{
    public int X { get; set; } = x;
    public int Y { get; set; } = y;
}
