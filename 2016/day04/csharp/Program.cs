using AoC.API;

int Part1((string name, string checksum)[] input)
{
    var realRoomSectorIdSum = 0;

    foreach (var room in input)
    {
        var sectorId = int.Parse(room.checksum.Split('[').First());
        var checksum = room.checksum.Split('[').Last();

        var letters = room.name.Replace("-", "").ToCharArray()
            .GroupBy(c => c)
            .Select(g => (g.Key, g.Count()))
            .OrderByDescending(g => g.Item2)
            .ThenBy(g => g.Key)
            .Select(g => g.Key)
            .Take(5)
            .ToArray();

        if (new string(letters) == checksum) { realRoomSectorIdSum += sectorId; }
    }

    return realRoomSectorIdSum;
}

int Part2((string name, string checksum)[] input)
{
    foreach (var room in input)
    {
        var sectorId = int.Parse(room.checksum.Split('[').First());
        var name = new string(room.name.Select(c => c == '-' ? ' ' : (char)((c % 'a' + sectorId) % 26 + 'a')).ToArray()).Split(' ');

        if (name.Contains("northpole") && name.Contains("object") && name.Contains("storage"))
        {
            return sectorId;
        }
    }

    throw new Exception("\"North Pole object storage\" could not be found");
}


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var input = (await session.GetInputLinesAsync()).Select(line => (line[..line.LastIndexOf('-')], line[(line.LastIndexOf('-') + 1)..].TrimEnd(']'))).ToArray();

Console.WriteLine(await session.SubmitAnswerAsync(1, Part1(input)));
Console.WriteLine(await session.SubmitAnswerAsync(2, Part2(input)));
