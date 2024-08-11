using System.Text;

var input = File.ReadAllLines("../input.txt").Select(ip => new IPv7(ip)).ToArray();

Console.WriteLine(input.Count(ip => ip.SupportsTLS()));
Console.WriteLine(input.Count(ip => ip.SupportsSSL()));

readonly struct IPv7
{
    public string Supernet { get; init; } = "";
    public string Hypernet { get; init; } = "";

    public IPv7(string ip)
    {
        var isHypernet = false;

        var supernet = new StringBuilder();
        var hypernet = new StringBuilder();

        for (var i = 0; i < ip.Length; i++)
        {
            if (ip[i] == '[')
            {
                supernet.Append(' ');
                isHypernet = true;
            }
            else if (ip[i] == ']')
            {
                hypernet.Append(' ');
                isHypernet = false;
            }
            else { (isHypernet ? hypernet : supernet).Append(ip[i]); }
        }

        Supernet = supernet.ToString();
        Hypernet = hypernet.ToString();
    }

    public readonly bool SupportsTLS() => !Hypernet.IsAbba() && Supernet.IsAbba();

    public readonly bool SupportsSSL()
    {
        var aba = new List<string>();

        for (var i = 0; i < Supernet.Length - 2; i++)
        {
            if (Supernet[i] == Supernet[i + 2] && Supernet[i] != Supernet[i + 1])
            {
                aba.Add($"{Supernet[i + 1]}{Supernet[i]}{Supernet[i + 1]}");
            }
        }

        return aba.Any(Hypernet.Contains);
    }
}

public static class StringExtensions
{
    public static bool IsAbba(this string s)
    {
        for (var i = 0; i < s.Length - 3; i++)
        {
            if (s[i] == s[i + 3] && s[i + 1] == s[i + 2] && s[i] != s[i + 1]) { return true; }
        }

        return false;
    }
}
