var lines = File.ReadAllLines("input.txt")
    .ToList();

var start = lines.First().LastIndexOf('S');

var beams = new Dictionary<int, long>();
beams[start] = 1;

foreach (var line in lines.Skip(1))
{
    var newBeams = new Dictionary<int, long>();

    foreach (var (beam, timelines) in beams)
    {
        if (line[beam] == '^')
        {
            if (newBeams.ContainsKey(beam - 1))
            {
                newBeams[beam - 1] += timelines;
            }
            else
            {
                newBeams[beam - 1] = timelines;
            }

            if (newBeams.ContainsKey(beam + 1))
            {
                newBeams[beam + 1] += timelines;
            }
            else
            {
                newBeams[beam + 1] = timelines;
            }
        }
        else
        {
            if (newBeams.ContainsKey(beam))
            {
                newBeams[beam] += timelines;
            }
            else
            {
                newBeams[beam] = timelines;
            }
        }
    }

    beams = newBeams;
}


var answer = beams.Values.Sum();

Console.WriteLine("answer: " + answer);
