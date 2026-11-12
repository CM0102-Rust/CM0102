import re,sys,glob
from collections import Counter
names={}
for f in glob.glob('referencias/cm0102-function-map/map/*.md'):
    for l in open(f,encoding='utf-8'):
        m=re.match(r'\| (0x[0-9a-f]+) \| [^|]*\| `([^`]*)`',l)
        if m: names[int(m.group(1),16)]=m.group(2)
ins=[]
for l in open(sys.argv[1],encoding='utf-8'):
    a=int(l[:8],16)
    if a>=0x5d590d: break
    ins.append((a,l[10:].strip()))
addrs=[a for a,_ in ins]
idx={a:i for i,a in enumerate(addrs)}
jumps=[]
for a,t in ins:
    m=re.match(r'j\w+\s+0x([0-9a-f]+)',t)
    if m:
        d=int(m.group(1),16)
        if 0x5ce33c<=d<0x5d5864 and 0x5ce33c<=a<0x5d5864: jumps.append((a,d))
long=sorted(jumps,key=lambda j:-abs(j[1]-j[0]))[:15]
for s,d in long: print('long',hex(s),hex(d),abs(d-s))
thr=int(sys.argv[2]) if len(sys.argv)>2 else 0
diff=[0]*(len(addrs)+1)
for s,d in jumps:
    lo,hi=min(s,d),max(s,d)
    if hi-lo>thr: continue
    i=idx[lo]+1; j=idx[hi]
    if i<j: diff[i]+=1; diff[j]-=1
c=0; pts=[]
for i,a in enumerate(addrs):
    c+=diff[i]
    if 0x5ce33c<a<0x5d5864 and c==0: pts.append(a)
segs=[]; start=0x5ce33c
for p in pts:
    if p-start>0x100: segs.append((start,p)); start=p
segs.append((start,0x5d5864))
for s,e in segs:
    calls=[names.get(int(m.group(1),16),hex(int(m.group(1),16))) for a,t in ins if s<=a<e for m in [re.match(r'call\s+0x([0-9a-f]+)',t)] if m]
    cc=Counter(calls)
    print(hex(s),hex(e),e-s,'|', ', '.join(f'{k}x{v}' if v>1 else k for k,v in cc.items())[:300])
