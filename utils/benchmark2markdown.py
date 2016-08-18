import sys

fin = sys.argv[1]
fout = sys.argv[2]

prefix = 'test unstable::bench_'

table = {}
known_a = set()

with open(fin) as f:
    for line in f:
        line = line.strip()
        if line.startswith(prefix):
            content = line[len(prefix):]
            s, t, a = content.split(' ')[0].split('_')
            x = int(content.split(':')[1].strip().split(' ')[0].replace(',', ''))

            if s not in table:
                table[s] = {}
            if t not in table[s]:
                table[s][t] = {}
            table[s][t][a] = x
            known_a.add(a)

with open(fout, 'w') as f:
    for s in sorted(table.keys()):
        f.write('{}:\n'.format(s))
        for t in sorted(table[s].keys()):
            f.write('//! | `{}` |'.format(t))
            best = min(table[s][t].values())
            for a in sorted(known_a):
                x = table[s][t][a]
                if x == best:
                    f.write(' **`{0:,}`** |'.format(x))
                else:
                    f.write(' `{0:,}` |'.format(x))
            f.write('\n')
