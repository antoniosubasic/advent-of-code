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

var input = File.ReadAllLines("../input.txt").Select(line => (line[..line.LastIndexOf('-')], line[(line.LastIndexOf('-') + 1)..].TrimEnd(']'))).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
