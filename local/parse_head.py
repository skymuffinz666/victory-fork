import re, collections
p = r'D:\AIProjects\GTA6PC\c_head_stderr.txt'
clean = [re.sub(r'\x1b\[[0-9;]*m', '', l) for l in open(p, encoding='utf-8', errors='replace').read().splitlines()]
msgs = [l for l in clean if 'victory_gpu::host: D3D12:' in l]
print('debug messages', len(msgs))
c = collections.Counter()
for l in msgs:
    s = re.sub(r'"[0-9a-f]+"', '"H"', l)
    s = re.sub(r'id=\d+ severity=\d+', 'id=N severity=N', s)
    c[s[23:190]] += 1
for s, n in c.most_common(25):
    print(n, '|', s)
