long answer = 0;

var instructions = File.ReadAllText("input.txt")
    .Split(',')
    .ToList();

foreach (var range in instructions)
{
    var rangeSplit = range.Split('-');

    var start = Int64.Parse(rangeSplit[0]);
    var end = Int64.Parse(rangeSplit[1]);

    for (long i = start; i <= end; i++)
    {
        if (!IsValidId(i))
        {
            answer += i;
        }
    }

}

Console.WriteLine(answer);

bool IsValidId(long id)
{
    var idString = id.ToString();

    var maxSequenceLength = idString.Length / 2;

    for (int size = 1; size <= maxSequenceLength; size++)
    {
        if (idString.Length % size != 0)
        {
            continue;
        }

        var sequences = new HashSet<string>();

        for (int i = 0; i < idString.Length; i += size)
        {
            sequences.Add(idString[i..(i + size)]);
        }

        if (sequences.Count == 1)
        {
            return false;
        }

    }

    return true;
}