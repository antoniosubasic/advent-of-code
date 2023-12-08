using AoC.API;

int Part1(int[][] input)
{
    var possible = 0;

    foreach (var triangle in input)
    {
        Array.Sort(triangle);
        if (triangle[0] + triangle[1] > triangle[2]) { possible++; }
    }

    return possible;
}

int Part2(int[][] input)
{
    var possible = 0;

    for (var i = 0; i < input.Length; i += 3)
    {
        for (var j = 0; j < 3; j++)
        {
            var triangle = new[] { input[i][j], input[i + 1][j], input[i + 2][j] };
            Array.Sort(triangle);
            if (triangle[0] + triangle[1] > triangle[2]) { possible++; }
        }
    }

    return possible;
}


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var input = (await session.GetInputLinesAsync()).Select(line => new[] { line[..5], line[7..10], line[10..] }.Select(int.Parse).ToArray());

Console.WriteLine(await session.SubmitAnswerAsync(1, Part1([.. input])));
Console.WriteLine(await session.SubmitAnswerAsync(2, Part2([.. input])));
