var answer = 0;

var instructions = File.ReadAllLines("input.txt")
    .ToList();

foreach (var line in instructions)
{
    var first = 0;
    var second = 0;

    foreach (var joltage in line.Take(line.Length - 1))
    {
        if (joltage > first)
        {
            first = joltage;
            second = 0;
            continue;
        }

        if (joltage > second)
        {
            second = joltage;
        }
    }

    var lastJoltage = line.Last();
    if (lastJoltage > second)
    {
        second = lastJoltage;
    }

    var bank = (first - '0') * 10 + (second - '0');

    answer += bank;
}

Console.WriteLine(answer);