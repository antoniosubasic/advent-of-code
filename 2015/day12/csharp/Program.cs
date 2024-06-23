using System.Text.Json;
using System.Text.RegularExpressions;

int Part1(string input) => new Regex(@"-?\d+").Matches(input).Select(match => int.Parse(match.Value)).Sum();

int ProcessJsonObject(JsonElement element)
{
    if (element.ValueKind is JsonValueKind.Object)
    {
        foreach (var property in element.EnumerateObject())
        {
            if (property.Value.ValueKind is JsonValueKind.String && property.Value.GetString() == "red") { return 0; }
        }
    }
        
    return element.ValueKind switch
    {
        JsonValueKind.Object => element.EnumerateObject().Select(property => ProcessJsonObject(property.Value)).Sum(),
        JsonValueKind.Array => element.EnumerateArray().Select(ProcessJsonObject).Sum(),
        JsonValueKind.Number => element.GetInt32(),
        _ => 0
    };
}

int Part2(string input)
{
    using (JsonDocument document = JsonDocument.Parse(input))
    {
        return ProcessJsonObject(document.RootElement);
    }
}

var input = File.ReadAllText("../input.txt");

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
