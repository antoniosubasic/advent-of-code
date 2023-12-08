using AoC.API;

long Part1(List<Hand> input)
{
    input.Sort((hand1, hand2) => hand1.CompareTo(hand2, 1));
    return input.Select((hand, index) => hand.Bid * (index + 1)).Sum();
}

long Part2(List<Hand> input)
{
    input.Sort((hand1, hand2) => hand1.CompareTo(hand2, 2));
    return input.Select((hand, index) => hand.Bid * (index + 1)).Sum();
}


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var input = (await session.GetInputLinesAsync()).Select(line => new Hand(line.Split(' ')));

Console.WriteLine(await session.SubmitAnswerAsync(1, Part1([.. input])));
Console.WriteLine(await session.SubmitAnswerAsync(2, Part2([.. input])));


class Hand(string[] hand)
{
    public char[] Cards { get; private set; } = hand[0].ToCharArray();
    public uint Bid { get; } = uint.Parse(hand[1]);

    public CardStrength GetStrength()
    {
        var cardCount = new Dictionary<char, int>();

        foreach (var card in Cards)
        {
            if (cardCount.ContainsKey(card)) { cardCount[card]++; }
            else { cardCount.Add(card, 1); }
        }

        if (cardCount.Any(card => card.Value == 5)) { return CardStrength.FiveOfAKind; }
        else if (cardCount.Any(card => card.Value == 4)) { return CardStrength.FourOfAKind; }
        else if (cardCount.Any(card => card.Value == 3))
        {
            return cardCount.Any(card => card.Value == 2) ? CardStrength.FullHouse : CardStrength.ThreeOfAKind;
        }
        else if (cardCount.Any(card => card.Value == 2))
        {
            return cardCount.Count(card => card.Value == 2) == 2 ? CardStrength.TwoPair : CardStrength.OnePair;
        }
        else { return CardStrength.HighCard; }
    }

    public CardStrength GetBestStrength()
    {
        if (!Cards.Contains('J')) { return GetStrength(); }
        else
        {
            var jokerCount = Cards.Count(card => card == 'J');
            var noneJoker = Cards.Where(card => card != 'J').ToArray();
            var noneJokerDistinct = noneJoker.Distinct().Count();

            if (jokerCount is 4 or 5 || noneJokerDistinct == 1) { return CardStrength.FiveOfAKind; }
            else if (jokerCount is 3) { return CardStrength.FourOfAKind; }
            else if (jokerCount is 2) { return noneJokerDistinct == 2 ? CardStrength.FourOfAKind : CardStrength.ThreeOfAKind; }
            else if (jokerCount is 1)
            {
                if (noneJokerDistinct == 2)
                {
                    if (noneJoker.Count(card => card == noneJoker[0]) == 3 || noneJoker.Count(card => card == noneJoker[1]) == 3) { return CardStrength.FourOfAKind; }
                    else { return CardStrength.FullHouse; }
                }
                else if (noneJokerDistinct == 3) { return CardStrength.ThreeOfAKind; }
                else { return CardStrength.OnePair; }
            }
            else { return GetStrength(); }
        }
    }

    public int CompareTo(Hand hand2, int part)
    {
        char[] cards = part == 1 ? ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'] : ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
        var hand1Strength = part == 1 ? GetStrength() : GetBestStrength();
        var hand2Strength = part == 1 ? hand2.GetStrength() : hand2.GetBestStrength();

        if (hand1Strength > hand2Strength) { return 1; }
        else if (hand1Strength < hand2Strength) { return -1; }
        else
        {
            for (int j = 0; j < Cards.Length; j++)
            {
                var card1Strength = Array.IndexOf(cards, Cards[j]);
                var card2Strength = Array.IndexOf(cards, hand2.Cards[j]);

                if (card1Strength > card2Strength) { return 1; }
                else if (card1Strength < card2Strength) { return -1; }
            }
        }

        return 0;
    }
}

enum CardStrength
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
