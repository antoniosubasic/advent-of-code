using AoC.API;

int Part1(Game[] input)
{
    var idsSum = 0;

    foreach (var game in input)
    {
        var valid = true;

        foreach (var subset in game.CubeSubsets)
        {
            valid = subset.All(cube => cube.amount <= (int)cube.color);
            if (!valid) { break; }
        }

        if (valid) { idsSum += game.Id; }
    }

    return idsSum;
}

int Part2(Game[] input) => input.Sum(game => game.GetPowerOf(Color.Red) * game.GetPowerOf(Color.Green) * game.GetPowerOf(Color.Blue));


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var input = (await session.GetInputLines())
    .Select(line =>
    {
        var splitted = line.Split(':');

        var game = int.Parse(splitted[0].Split(' ')[^1]);
        var subsets = new List<(int, Color)[]>();

        foreach (var subset in splitted[1].Trim().Split(';'))
        {
            var cubes = new List<(int, Color)>();

            foreach (var cubeItem in subset.Split(','))
            {
                var cubeTuple = cubeItem.Trim().Split(' ');

                cubes.Add(
                    (
                        int.Parse(cubeTuple[0]),
                        cubeTuple[1] switch
                        {
                            "blue" => Color.Blue,
                            "red" => Color.Red,
                            "green" => Color.Green,
                            _ => throw new Exception("Unknown color")
                        }
                    )
                );
            }

            subsets.Add(cubes.ToArray());
        }
        
        return new Game(game, subsets.ToArray());
    }).ToArray();

Console.WriteLine(await session.SubmitAnswer(1, Part1(input)));
Console.WriteLine(await session.SubmitAnswer(2, Part2(input)));


record Game(int Id, (int amount, Color color)[][] CubeSubsets)
{
    public int GetPowerOf(Color color) => CubeSubsets.Max(subset => subset.Where(cube => cube.color == color).Sum(cube => cube.amount));
}

enum Color
{
    Red = 12,
    Green = 13,
    Blue = 14,
}
