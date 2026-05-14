#!/usr/bin/env python3
import csv
import os
from collections import Counter, defaultdict

root = '/Users/jasonpoindexter/Documents/GitHub/theft-kit'
skills = [n for n in os.listdir(root) if os.path.isfile(os.path.join(root, n, 'SKILL.md'))]

prefix_counter = Counter()
for s in skills:
    p = s.split('-')[0] if '-' in s else s
    prefix_counter[p] += 1

families = defaultdict(list)
for s in skills:
    toks = s.split('-')
    key = '-'.join(toks[:2]) if len(toks) >= 2 else s
    families[key].append(s)

out = os.path.join(root, 'STREAMLINE_REPORT.md')
with open(out, 'w') as f:
    f.write('# Streamline Report\n\n')
    f.write(f'Total skills: {len(skills)}\n\n')
    f.write('## Largest Prefix Groups\n\n')
    for k, v in prefix_counter.most_common(20):
        f.write(f'- `{k}`: {v}\n')
    f.write('\n## Large Families (4+)\n\n')
    large = sorted((k, v) for k, v in families.items() if len(v) >= 4)
    for k, v in large:
        f.write(f'- `{k}`: {len(v)}\n')

csv_out = os.path.join(root, 'SKILL_GROUPS.csv')
with open(csv_out, 'w', newline='') as c:
    w = csv.writer(c)
    w.writerow(['skill', 'prefix', 'family'])
    for s in sorted(skills):
        toks = s.split('-')
        prefix = toks[0] if len(toks) > 1 else s
        family = '-'.join(toks[:2]) if len(toks) > 1 else s
        w.writerow([s, prefix, family])

print(out)
print(csv_out)
