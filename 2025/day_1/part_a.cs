var dial = 50;
var password = 0;

var instructions = File.ReadAllLines("input.txt")
    .Select(line => (turn: line[0], steps: int.Parse(line[1..])))
    .ToList();

foreach (var (line, steps) in instructions)
{
    if (line == 'R')
        dial += steps;
    else
        dial -= steps;

    dial %= 100;

    if (dial < 0)
        dial = 100 + dial;

    if (dial == 0)
        password++;
}

Console.WriteLine(password);