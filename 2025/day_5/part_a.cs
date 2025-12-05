var lines = File.ReadAllLines("input.txt")
    .ToList();

var freshIds = new List<(long low, long high)>();

var answer = 0;

var isFreshIngredients = true;
foreach (var line in lines)
{
    if (isFreshIngredients && line.Length == 0)
    {
        isFreshIngredients = false;
        continue;
    }


    if (isFreshIngredients)
    {
        var rangeSplit = line.Split('-');

        var start = Int64.Parse(rangeSplit[0]);
        var end = Int64.Parse(rangeSplit[1]);

        freshIds.Add((start, end));

    }
    else
    {
        var id = Int64.Parse(line);

        foreach (var (low, high) in freshIds)
        {
            if (id >= low && id <= high)
            {
                answer++;
                break;
            }
        }
    }
}

Console.WriteLine("answer: " + answer);
