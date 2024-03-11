int Part1(Scratchcard[] input)
{
    var sum = 0;

    foreach (var card in input)
    {
        var tempSum = 0;
        for (var i = 0; i < card.MatchingNumbers; i++)
        {
            tempSum += i == 0 ? 1 : tempSum;
        }

        sum += tempSum;
    }

    return sum;
}

int Part2(Scratchcard[] input)
{
    var cardMap = input.ToDictionary(card => card.Id, card => 1);

    for (var i = 0; i < input.Length; i++)
    {
        var currentCardAmount = cardMap[input[i].Id];
        var cardsToCopy = input[(i + 1)..(i + input[i].MatchingNumbers + 1)];

        foreach (var card in cardsToCopy)
        {
            cardMap[card.Id] += currentCardAmount;
        }
    }

    return cardMap.Values.Sum();
}


var input = File.ReadAllLines("../input.txt")
    .Select(line =>
    {
        var split = line.Split(" | ");

        var given = split[0].Split(": ");

        var id = int.Parse(given[0].Split(' ').Last());
        var winningNumbers = given[1].Split(' ').Where(str => str != "").Select(int.Parse).ToArray();

        var myNumbers = split[1].Split(' ').Where(str => str != "").Select(int.Parse).ToArray();

        return new Scratchcard(id, winningNumbers, myNumbers);
    }).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));


struct Scratchcard(int id, int[] winningNumbers, int[] myNumbers)
{
    public int Id { get; init; } = id;
    public readonly int MatchingNumbers => winningNumbers.Intersect(myNumbers).Count();
}
