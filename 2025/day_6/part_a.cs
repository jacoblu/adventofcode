var lines = File.ReadAllLines("input.txt")
    .ToList();

var operations = lines.Last().Where(c => c != ' ').ToList();

var partials = new long[operations.Count];
for (int i = 0; i < operations.Count; i++)
{
    if (operations[i] == '*')
    {
        partials[i] = 1;
    }
}

foreach (var line in lines.Take(lines.Count - 1))
{
    var numbers = line.Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries).Select(n => long.Parse(n)).ToList();

    for (int i = 0; i < numbers.Count; i++)
    {
        if (operations[i] == '*')
        {
            partials[i] *= numbers[i];
        }
        else
        {
            partials[i] += numbers[i];

        }
    }
}

long answer = partials.Sum();

Console.WriteLine("answer: " + answer);
