﻿using AoC.API;

int Part1(Game[] input) => input.Sum(game => game.CubeSubsets.All(subset => subset.All(cube => cube.amount <= (int)cube.color)) ? game.Id : 0);

int Part2(Game[] input) => input.Sum(game => game.GetMaxNumberOfCubes(Color.Red) * game.GetMaxNumberOfCubes(Color.Green) * game.GetMaxNumberOfCubes(Color.Blue));


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
    public int GetMaxNumberOfCubes(Color color) => CubeSubsets.Max(subset => subset.Sum(cube => cube.color == color ? cube.amount : 0));
}

enum Color
{
    Red = 12,
    Green = 13,
    Blue = 14,
}
