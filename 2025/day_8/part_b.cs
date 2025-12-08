var lines = File.ReadAllLines("input.txt").ToList();

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

var target = junctions.Count();
foreach (var (_, j1, j2) in sortedJunctions)
{
    var circuitSize = j1.Connect(j2);

    if (circuitSize >= target)
    {
        Console.WriteLine("answer: " + j1.X * j2.X);
        break;
    }
}

class Circuit
{
    public IEnumerable<Junction> Junctions => junctions;
    private HashSet<Junction> junctions = new HashSet<Junction>();

    public Circuit(Junction junction)
    {
        junctions.Add(junction);
    }

    public int Merge(Circuit other)
    {
        junctions.UnionWith(other.Junctions);

        foreach (var junction in junctions)
        {
            junction.Circuit = this;
        }

        return junctions.Count;
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

    public int Connect(Junction other)
    {
        return Circuit.Merge(other.Circuit);
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