import sys,re,collections
def load(f):
    out=[]
    for l in open(f).read().splitlines():
        m=re.match(r'(\d+) s=\[(.*?)\] a=\[(.*?)\] ds=(\w+) dp=(\w+)(.*)',l)
        if not m: out.append(None); continue
        orc={}
        for part in m.group(6).split(' | ')[1:]:
            k,res=part.split(' ',1); orc[k]=res
        out.append((set(t.strip() for t in m.group(3).split(',') if t.strip()),orc))
    return out
A=load(sys.argv[1]); B=load(sys.argv[2])
for name,X in [('base',A),('proto',B)]:
    c=collections.Counter(r if r=='hit' else 'miss' for x in X if x for r in x[1].values())
    print(name,'collected:',dict(c))
c=collections.Counter()
for x,y in zip(A,B):
    if not x: continue
    for k in y[0]-x[0]: c['extra_'+y[1][k].split()[0]]+=1
    for k in x[0]-y[0]: c['MISSING']+=1
print('proto vs base:',dict(c))
