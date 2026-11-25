"""compact.py all.asm out.txt: the disassembly shortened and annotated (globals, .rdata constants, callees)."""
import re, sys, struct
R = r'D:\Sistemas\CM0102\rust\src'
names = {}
for f in (R + r'\motor\globals.rs', R + r'\motor\rdata_consts.rs'):
    for n, a, c in re.findall(r'const (\w+): usize = (0x[0-9a-f]+);(?: // (.*))?', open(f, encoding='utf-8').read()):
        names.setdefault(int(a, 16), n + ('{%s}' % c.split('= ')[-1] if c and '=' in c else ''))
fns = open(R + r'\motor\exe_fns.rs', encoding='utf-8').read()
cal = {int(a, 16): n[3:].lower() for n, a in re.findall(r'pub const (FN_\w+): usize = (0x[0-9a-f]+);', fns)}
motor = open(R + r'\motor.rs', encoding='utf-8').read()
for a, n in re.findall(r'Prologue \{ address: (0x[0-9a-f]+), bytes: &\[[^\]]*\] \} => (\w+)', motor):
    cal.setdefault(int(a, 16), 'R:' + n)
cal.update({0x945640: 'ftol', 0x945ff0: 'pow?', 0x90cfc0: 'random'})
out = []
for l in open(sys.argv[1]):
    m = re.match(r'00([0-9a-f]{6})  (\S+)\s*(.*)', l.rstrip())
    if not m:
        continue
    a, mn, ops = m.groups()
    ops = ops.replace('dword ptr ', 'd').replace('word ptr ', 'w').replace('byte ptr ', 'b').replace('qword ptr ', 'q').replace('tbyte ptr ', 't')
    def rep(mm):
        v = int(mm.group(1), 16)
        if v in names:
            return names[v]
        return mm.group(0)
    ops = re.sub(r'0x([0-9a-f]{6,7})\b', rep, ops)
    if mn == 'call':
        try:
            v = int(m.group(3).strip(), 16)
            ops = '%s %s' % (m.group(3).strip(), cal.get(v, ''))
        except ValueError:
            pass
    out.append('%s %s %s' % (a, mn, ops))
open(sys.argv[2], 'w').write('\n'.join(out) + '\n')
