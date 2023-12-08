using AoC.API;

int Part1(Network input)
{
    int steps;

    for (steps = 0; input.Current != "ZZZ"; steps++)
    {
        input.Current = input.Instructions[steps % input.Instructions.Length] switch
        {
            'L' => input.Nodes[input.Current].left,
            'R' => input.Nodes[input.Current].right,
            _ => input.Current
        };
    }

    return steps;
}

long Part2(Network input)
{
    var currentNodesSteps = input.CurrentNodes.Select(node =>
    {
        input.Current = node;
        int steps;

        for (steps = 0; input.Current[^1] != 'Z'; steps++)
        {
            input.Current = input.Instructions[steps % input.Instructions.Length] switch
            {
                'L' => input.Nodes[input.Current].left,
                'R' => input.Nodes[input.Current].right,
                _ => input.Current
            };
        }

        return steps;
    }).ToArray();

    long lcm = currentNodesSteps[0];
    for (var i = 1; i < currentNodesSteps.Length; i++) { lcm = LCM(lcm, currentNodesSteps[i]); }
    return lcm;
}

long LCM(long a, long b) => a * b / GCD(a, b);

long GCD(long a, long b)
{
    while (b != 0)
    {
        var temp = b;
        b = a % b;
        a = temp;
    }

    return a;
}


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var rawInput = await session.GetInputLinesAsync();
var input = new Network(rawInput.First(), rawInput[2..]);

Console.WriteLine(await session.SubmitAnswerAsync(1, Part1(input)));
Console.WriteLine(await session.SubmitAnswerAsync(2, Part2(input)));


class Network
{
    public string Current { get; set; }
    public string[] CurrentNodes { get; set; }
    public string Instructions { get; }
    public Dictionary<string, (string left, string right)> Nodes { get; } = [];

    public Network(string instructions, string[] nodes)
    {
        Instructions = instructions;
        Current = "AAA";
        Nodes = nodes.ToDictionary(
            node => node.Split(" = ")[0],
            node => (node.Split(" = ")[1].Trim('(', ')').Split(", ")[0], node.Split(" = ")[1].Trim('(', ')').Split(", ")[1])
        );
        CurrentNodes = Nodes.Keys.Where(node => node[^1] == 'A').ToArray();
    }
}
