int Part1(string[][] input) => input.Count(passport => passport.Length == 8 || (passport.Length == 7 && !passport.Any(field => field.StartsWith("cid"))));

int Part2(string[][] input)
{
    var valid = 0;

    foreach (var passport in input)
    {
        if (
            (passport.Length == 8 || (passport.Length == 7 && !passport.Any(field => field.StartsWith("cid"))))
            && passport.All(field =>
            {
                var key = field.Split(':')[0];
                var value = field.Split(':')[1];

                return key switch
                {
                    "byr" => int.TryParse(value, out var byr) && byr >= 1920 && byr <= 2002,
                    "iyr" => int.TryParse(value, out var iyr) && iyr >= 2010 && iyr <= 2020,
                    "eyr" => int.TryParse(value, out var eyr) && eyr >= 2020 && eyr <= 2030,
                    "hgt" => value.EndsWith("cm") ? (int.TryParse(value[..^2], out var hgtCm) && hgtCm >= 150 && hgtCm <= 193) : (value.EndsWith("in") && int.TryParse(value[..^2], out var hgtIn) && hgtIn >= 59 && hgtIn <= 76),
                    "hcl" => value.Length == 7 && value[0] == '#' && value[1..].All(c => char.IsDigit(c) || (c >= 'a' && c <= 'f')),
                    "ecl" => new[] { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" }.Contains(value),
                    "pid" => value.Length == 9 && value.All(char.IsDigit),
                    _ => true
                };
            })
        )
        {
            valid++;
        }
    }

    return valid;
}

var input = File.ReadAllText("../input.txt").Split("\n\n").Select(passport => passport.Replace('\n', ' ').Split(' ')).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
