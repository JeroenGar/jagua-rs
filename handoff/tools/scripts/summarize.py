import sys, glob, re, os, statistics as st
d = sys.argv[1]
res = {}
for f in sorted(glob.glob(os.path.join(d, '*_*.txt'))):
    w = os.path.basename(f).split('_')[0]
    for line in open(f):
        m = re.match(r'test (\S+) \.\.\. bench:\s+([\d,]+) ns/iter', line)
        if m:
            res.setdefault(m.group(1), {}).setdefault(w, []).append(int(m.group(2).replace(',', '')))
print(f"{'bench':45s} {'base (us)':>18s} {'var (us)':>18s} {'change':>8s}")
for k, v in res.items():
    b = v.get('base', []); a = v.get('var', [])
    fmt = lambda x: f"{st.mean(x)/1e3:8.1f}±{(st.stdev(x) if len(x)>1 else 0)/1e3:5.1f}" if x else '-'
    ch = f"{(st.mean(a)/st.mean(b)-1)*100:+6.1f}%" if a and b else ''
    print(f"{k:45s} {fmt(b):>18s} {fmt(a):>18s} {ch:>8s}")
