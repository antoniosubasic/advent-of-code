using System.Text.RegularExpressions;

int Part1(Record[] input, out Dictionary<int, List<int>> guards)
{
    guards = [];

    for (var i = 0; i < input.Length; i++)
    {
        switch (input[i].Operation)
        {
            case Operation.BeginShift:
                if (!guards.ContainsKey(input[i].GuardId)) { guards[input[i].GuardId] = []; }
                break;

            case Operation.FallAsleep:
                var sleep = input[i].DateTime.Minute;
                var wake = input[++i].DateTime.Minute;
                guards[input[i].GuardId].AddRange(Enumerable.Range(sleep, wake - sleep));
                break;
        }
    }

    var guardWithMostSleep = guards.OrderBy(guard => guard.Value.Count).Last();
    var minuteMostAsleep = guardWithMostSleep.Value.Distinct().OrderBy(minute => guardWithMostSleep.Value.Count(min => min == minute)).Last();

    return guardWithMostSleep.Key * minuteMostAsleep;
}

int Part2(Record[] input, Dictionary<int, List<int>> guards)
{
    var guardsMostAsleep = new Dictionary<int, int>();

    foreach (var guard in guards)
    {
        if (guard.Value.Count != 0)
        {
            var minuteMostAsleep = guard.Value.Distinct().OrderBy(minute => guard.Value.Count(min => min == minute)).Last();
            guardsMostAsleep[guard.Key] = minuteMostAsleep;
        }
    }

    var guardMostAsleep = guardsMostAsleep.OrderBy(guard => guards[guard.Key].Count(minute => minute == guard.Value)).Last();
    return guardMostAsleep.Key * guardMostAsleep.Value;
}


var input = File.ReadAllLines("../input.txt").Order().Select(Record.Parse).ToArray();

Console.WriteLine(Part1(input, out var guards));
Console.WriteLine(Part2(input, guards));


enum Operation
{
    BeginShift,
    FallAsleep,
    WakeUp
}

record Record(DateTime DateTime, int GuardId, Operation Operation)
{
    private static int _previousGuardId = 0;

    public static Record Parse(string input)
    {
        var match = Regex.Match(input, @"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.+)");
        var dateTime = DateTime.Parse(match.Groups[1].Value);
        var operation = match.Groups[2].Value switch
        {
            "falls asleep" => Operation.FallAsleep,
            "wakes up" => Operation.WakeUp,
            _ => Operation.BeginShift
        };

        if (operation == Operation.BeginShift)
        {
            var guardId = int.Parse(Regex.Match(match.Groups[2].Value, @"Guard #(\d+) begins shift").Groups[1].Value);
            return new(dateTime, _previousGuardId = guardId, Operation.BeginShift);
        }
        else { return new(dateTime, _previousGuardId, operation); }
    }
}
