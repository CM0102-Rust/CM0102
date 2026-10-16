"""Placar da migracao. Uso: python fronteiras.py [--candidatas]

Fronteiras reais das funcoes: inicio = alvo de call (ou ja portado); fim = proximo inicio.
Imprime cobertura por modulo e as candidatas (motor) com chamadas ainda nao portadas."""
import csv, re, glob, sys
import os
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # raiz do projeto
d = open(R + r'\jogo\cm0102.exe', 'rb').read()
TXT0, TXT1 = 0x401000, 0x401000 + 0x55c000
alvos = set()
codigo = d[TXT0 - 0x400000:TXT1 - 0x400000]
# varredura linear barata: E8 rel32 com alvo dentro do .text
for i in range(len(codigo) - 5):
    if codigo[i] == 0xE8:
        rel = int.from_bytes(codigo[i + 1:i + 5], 'little', signed=True)
        t = TXT0 + i + 5 + rel
        if TXT0 <= t < TXT1:
            alvos.add(t)
port = set()
for f in glob.glob(R + r'\rust\src\*.rs'):
    t = open(f, encoding='utf-8').read()
    # so a tabela PROLOGOS (copias em Rust); as portas de gravacao (PROLOGO_xxx, PROLOGOS_POS) nao contam
    if 'const PROLOGOS:' in t:
        i = t.index('const PROLOGOS:')
        t = t[i:t.index('\n];', i)]
    else:
        t = re.sub(r'const PROLOGO_[0-9A-F]+: Prologo = Prologo [^;]*;', '', t)
    port |= set(int(x, 16) for x in re.findall(r'endereco:\s*0x([0-9a-f]+)', t))
rows = [r for r in csv.reader(open(R + r'\docs\mapa_funcoes.csv', encoding='utf-8')) if r and r[0].startswith('0x')]
mapa = {int(r[0], 16): (int(r[1]), r[2].split('/')[-1] if r[2] else '(sem)') for r in rows}
inicios = sorted(a for a in mapa if a in alvos or a in port)
fim_txt = max(a + n for a, (n, _) in mapa.items())
ext = {}
for k, a in enumerate(inicios):
    b = inicios[k + 1] if k + 1 < len(inicios) else fim_txt
    ext[a] = b - a
mod = {a: mapa[a][1] for a in inicios}
tot, cov = {}, {}
for a in inicios:
    m = mod[a]
    tot.setdefault(m, [0, 0]); cov.setdefault(m, [0, 0])
    tot[m][0] += 1; tot[m][1] += ext[a]
    if a in port:
        cov[m][0] += 1; cov[m][1] += ext[a]
motor = [m for m in tot if m.startswith('match_')]
for m in sorted(motor, key=lambda m: -tot[m][1]):
    print(f"{m:20s} {cov[m][0]:4d}/{tot[m][0]:<4d} funcoes {cov[m][1]/1024:6.1f}/{tot[m][1]/1024:6.1f} KB {100*cov[m][1]/tot[m][1]:5.1f}%")
c = sum(cov[m][1] for m in motor); t = sum(tot[m][1] for m in motor)
ct = sum(v[1] for v in cov.values()); tt = sum(v[1] for v in tot.values())
print(f"motor {c/1024:.0f}/{t/1024:.0f} KB = {100*c/t:.1f}%   exe {ct/1024:.0f}/{tt/1024:.0f} KB = {100*ct/tt:.2f}%  ({len(port)} copias)")
sm = [m for m in motor if m != 'match_screens.cpp']
c2 = sum(cov[m][1] for m in sm); t2 = sum(tot[m][1] for m in sm)
print(f"motor sem telas {c2/1024:.0f}/{t2/1024:.0f} KB = {100*c2/t2:.1f}%")
if '--candidatas' in sys.argv:
    for a in inicios:
        if mod[a] in motor and mod[a] != 'match_screens.cpp' and a not in port:
            seg = d[a - 0x400000:a - 0x400000 + ext[a]]
            chama = set()
            for i in range(len(seg) - 5):
                if seg[i] == 0xE8:
                    t = a + i + 5 + int.from_bytes(seg[i + 1:i + 5], 'little', signed=True)
                    if t in alvos:
                        chama.add(t)
            falta = sorted(x for x in chama if x not in port and x < 0x940000)
            print(f"{a:#x} {ext[a]:6d} {mod[a]:18s} faltam {len(falta):2d} {' '.join(hex(x) for x in falta[:8])}")
