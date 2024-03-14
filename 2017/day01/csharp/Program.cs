int Part1And2(string input, int part)
{
    var captcha = 0;

    for (var i = 0; i < input.Length; i++)
    {
        var next = part == 1 ? i + 1 : i + input.Length / 2;

        if (input[i] == input[next % input.Length])
        {
            captcha += input[i] - '0';
        }
    }

    return captcha;
}

var input = File.ReadAllText("../input.txt");

Console.WriteLine(Part1And2(input, 1));
Console.WriteLine(Part1And2(input, 2));