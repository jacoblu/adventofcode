var lines = File.ReadAllLines("input.txt").ToList();
var pairs = 1000;

var junctions = new List<Junction>();

foreach (var line in lines)
{
    var coords = line.Split(',').Select(x => long.Parse(x)).ToList();

    var junction = new Junction(coords[0], coords[1], coords[2]);
    junctions.Add(junction);
}

var distances = new List<(double dist, Junction j1, Junction j2)>();
for (int i = 0; i < junctions.Count() - 1; i++)
{
    var junction = junctions[i];

    for (int j = i + 1; j < junctions.Count(); j++)
    {
        var otherJunction = junctions[j];
        var newDist = junction.Distance(otherJunction);

        distances.Add((newDist, junction, otherJunction));
    }
}

var sortedJunctions = distances
    .OrderBy(t => t.dist).ToList();

foreach (var (_, j1, j2) in sortedJunctions.Take(pairs))
{
    j1.Connect(j2);
}

var circuits = new HashSet<Circuit>();
foreach (var junction in junctions)
{
    circuits.Add(junction.Circuit);
}

var answer = circuits
    .OrderByDescending(x => x.Junctions.Count())
    .Select(g => g.Junctions.Count())
    .Take(3)
    .Aggregate(1, (acc, count) => acc * count);

Console.WriteLine("answer: " + answer);

class Circuit
{
    public IEnumerable<Junction> Junctions => junctions;
    private HashSet<Junction> junctions = new HashSet<Junction>();

    public Circuit(Junction junction)
    {
        junctions.Add(junction);
    }

    public void Merge(Circuit other)
    {
        junctions.UnionWith(other.Junctions);

        foreach (var junction in junctions)
        {
            junction.Circuit = this;
        }
    }
}

class Junction
{
    public long X => x;
    public long Y => y;
    public long Z => z;

    private readonly long x;
    private readonly long y;
    private readonly long z;

    public Junction(long x, long y, long z)
    {
        this.x = x;
        this.y = y;
        this.z = z;

        Circuit = new Circuit(this);
    }

    public Circuit Circuit { get; set; }

    public void Connect(Junction other)
    {
        Circuit.Merge(other.Circuit);
    }

    public double Distance(Junction other)
    {
        return Math.Sqrt(Math.Pow(Math.Abs(x - other.X), 2) + Math.Pow(Math.Abs(y - other.Y), 2) + Math.Pow(Math.Abs(z - other.Z), 2));
    }

    public override string ToString()
    {
        return $"({x},{y},{z})";
    }
}