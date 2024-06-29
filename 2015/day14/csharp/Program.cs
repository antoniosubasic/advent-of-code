using System.Text.RegularExpressions;

const int TIME = 2503;

var regex = new Regex(@"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds");
var input = File.ReadAllLines("../input.txt").Select(line =>
{
    var match = regex.Match(line);
    return new Reindeer(match.Groups[1].Value, (int.Parse(match.Groups[2].Value), int.Parse(match.Groups[3].Value)), int.Parse(match.Groups[4].Value));
}).ToList();

Console.WriteLine(input.Select(reindeer => reindeer.DistanceTravelled(TIME)).Max());

var points = new Dictionary<string, int>();

for (var time = 1; time <= TIME; time++)
{
    var raindeers = input.Select(reindeer => (name: reindeer.Name, distance: reindeer.DistanceTravelled(time))).ToList();
    var maxDistance = raindeers.Max(raindeer => raindeer.distance);

    foreach (var (name, _) in raindeers.Where(raindeer => raindeer.distance == maxDistance))
    {
        points[name] = points.GetValueOrDefault(name) + 1;
    }
}

Console.WriteLine(points.Values.Max());

record struct Reindeer(string Name, (int speed, int duration) Travel, int RestTime)
{
    public readonly int DistanceTravelled(int time)
    {
        var cycle = Travel.duration + RestTime;
        var cycles = time / cycle;
        var remaining = time % cycle;
        return cycles * Travel.speed * Travel.duration + Math.Min(Travel.duration, remaining) * Travel.speed;
    }
}
