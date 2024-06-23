bool IsValidPassword(char[] password)
{
    var hasIncreasingStraight = false;
    (int lastIndex, int count) twoPairs = (-2, 0);

    for (var i = 0; i < password.Length; i++)
    {
        if (password[i] is 'i' or 'o' or 'l') { return false; }
        else
        {
            if (i < password.Length - 2 && password[i] + 1 == password[i + 1] && password[i + 1] + 1 == password[i + 2]) { hasIncreasingStraight = true; }
            else if (i < password.Length - 1 && i > twoPairs.lastIndex + 1 && password[i] == password[i + 1])
            {
                twoPairs.count++;
                twoPairs.lastIndex = i;
            }
        }
    }

    return hasIncreasingStraight && twoPairs.count >= 2;
}

void IncreasePassword(char[] password)
{
    for (var i = password.Length - 1; i >= 0; i--)
    {
        if (password[i] == 'z') { password[i] = 'a'; }
        else
        {
            password[i]++;
            break;
        }
    }
}

char[] Part1And2(char[] input)
{
    while (!IsValidPassword(input))
    {
        var invalidCharFound = false;

        for (var i = 0; i < input.Length; i++)
        {
            if (invalidCharFound) { input[i] = 'a'; }
            else if (invalidCharFound = input[i] is 'i' or 'o' or 'l') { input[i] = (char)(input[i] + 1); }
        }

        if (!invalidCharFound) { IncreasePassword(input); }
    }

    return input;
}

var input = File.ReadAllText("../input.txt").ToCharArray();

var part1 = Part1And2(input);
Console.WriteLine(part1);

IncreasePassword(part1);
Console.WriteLine(Part1And2(part1));
