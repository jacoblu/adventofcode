var lines = File.ReadAllLines("input.txt")
    .ToList();

var start = lines.First().LastIndexOf('S');

var beams = new HashSet<int>();
beams.Add(start);

var splits = 0;
foreach (var line in lines.Skip(1))
{
    var newBeams = new HashSet<int>();

    foreach (var beam in beams)
    {
        if (line[beam] == '^')
        {
            newBeams.Add(beam - 1);
            newBeams.Add(beam + 1);
            splits++;
        }
        else
        {
            newBeams.Add(beam);
        }
    }

    beams = newBeams;
}


Console.WriteLine("answer: " + splits);
