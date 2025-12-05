var lines = File.ReadAllLines("input.txt")
    .ToList();

var freshIds = new List<Range>();

foreach (var line in lines)
{
    if (line.Length == 0)
    {
        break;
    }

    var rangeSplit = line.Split('-');

    var start = Int64.Parse(rangeSplit[0]);
    var end = Int64.Parse(rangeSplit[1]);

    var newRange = new Range(start, end);

    freshIds.Add(newRange);
}

freshIds = freshIds.OrderBy(r => r.Start).ToList();

var merged = new List<Range>();

for (int i = 0; i < freshIds.Count - 1; i++)
{
    var r1 = freshIds[i];
    var r2 = freshIds[i + 1];

    if (r1.Overlaps(r2))
    {
        r2.Merge(r1);
    }
    else
    {
        merged.Add(r1);
    }


}
merged.Add(freshIds.Last());

long answer = 0;
foreach (var r in merged)
{
    answer += r.Count();
}

Console.WriteLine(answer);

class Range(long start, long end)
{
    public long Start => start;
    public long End => end;

    public bool Overlaps(Range other)
    {
        return !(other.End < start || other.Start > end);
    }

    public void Merge(Range other)
    {
        start = Math.Min(start, other.Start);
        end = Math.Max(end, other.End);
    }

    public long Count()
    {
        return end - start + 1;
    }

    public override string ToString()
    {
        return $"{start.ToString()}-{end.ToString()}";
    }
}
