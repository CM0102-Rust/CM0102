r"""Proximas rotinas do motor: as cobertas (executadas no jogo real) que ainda nao estao em Rust.

Uso: python candidatos.py                lista por dependencias pendentes e tamanho
     python candidatos.py --simulacao   idem, nos modulos da simulacao (docs/cobertura_simulacao.txt)
     python candidatos.py --competicoes idem, nas competicoes (docs/cobertura_competicoes.txt, competicoes.py)
     python candidatos.py --fora         chamadas pelo motor, fora da lista de cobertura
     python candidatos.py 0x6c51e0 ...   extensao real de cada uma e o que ela chama ainda em C

"Feitas" = enderecos com Prologue em rust/src/motor.rs. "Falta" = chamadas a rotinas ainda nao
reescritas; com falta 0, a rotina so depende do que ja esta provado (melhor candidata).
Neutras: o sorteio (0x90cfc0), _ftol (0x945640) e os tratadores de erro, chamados por endereco.
A lista de cobertura (docs/cobertura_motor.txt) vem de cobertura.py + simular.py --cobertura
(carreira com o motor ligado). O mapa tem pedacos desalinhados (enderecos "quebrados" com
tamanho pequeno no topo da lista): conferir o inicio com desmonta.py antes de escolher.
"""
import bisect
import csv
import os
import re
import sys

from capstone import CS_ARCH_X86, CS_MODE_32, Cs

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)
d = open(os.path.join(RAIZ, 'jogo', 'cm0102.exe'), 'rb').read()
rows = sorted((int(r['endereco'], 16), int(r['tamanho']), r['arquivo']) for r in csv.DictReader(open(os.path.join(RAIZ, 'docs', 'mapa_funcoes.csv'))))
starts = [r[0] for r in rows]
tam = {r[0]: r[1] for r in rows}
arq = {r[0]: r[2].split('/')[-1] for r in rows}
src = open(os.path.join(RAIZ, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
feitas = {int(x, 16) for x in re.findall(r'Prologue \{ address: (0x[0-9a-f]+)', src)}
neutras = {0x90cfc0, 0x945640, 0x5e8290, 0x90d130, 0x944c9f, 0x944cff, 0x5a8f60, 0x5cc7d0, 0x945934}
# --simulacao: os modulos da simulacao fora do motor e das telas (docs/cobertura_simulacao.txt, 17/10/2026)
# --competicoes: as rotinas de competicao que rodam com 8 ligas (docs/cobertura_competicoes.txt, competicoes.py, 03/11/2026)
LISTA = ('cobertura_simulacao.txt' if '--simulacao' in sys.argv else
         'cobertura_competicoes.txt' if '--competicoes' in sys.argv else 'cobertura_motor.txt')
cob = [int(l.split()[0], 16) for l in open(os.path.join(RAIZ, 'docs', LISTA)) if l.strip() and not l.startswith('#')]
md = Cs(CS_ARCH_X86, CS_MODE_32)


def func(a):
    i = bisect.bisect_right(starts, a) - 1
    return rows[i][0] if i >= 0 and a < rows[i][0] + max(rows[i][1], 1) else None


def chamadas(a, n=None):
    alvos, ind = set(), 0
    for ins in md.disasm(d[a - 0x400000:a - 0x400000 + (n or tam.get(a, 0))], a):
        if ins.mnemonic == 'call':
            if ins.op_str.startswith('0x'):
                t = int(ins.op_str, 16)
                alvos.add(func(t) or t)
            else:
                ind += 1
    return alvos, ind


def extensao(a):
    """Tamanho real: ate um ret alem de todos os saltos internos (o mapa pode errar)."""
    md.skipdata = True
    alvo_max = a
    for ins in md.disasm(d[a - 0x400000:a - 0x400000 + 0x8000], a):
        if ins.mnemonic.startswith('j') and ins.op_str.startswith('0x'):
            alvo_max = max(alvo_max, int(ins.op_str, 16))
        fim = ins.address + ins.size - a
        if ins.mnemonic in ('ret', 'retn') and ins.address >= alvo_max:
            return fim
        if ins.mnemonic == 'jmp' and ins.address >= alvo_max and not ins.op_str.startswith('0x'):
            return fim
    return None


def main():
    args = [x for x in sys.argv[1:] if not x.startswith('--')]
    if args:
        for x in args:
            a = int(x, 16)
            n = extensao(a)
            alvos, ind = chamadas(a, n)
            falta = sorted(t for t in alvos if t not in feitas and t not in neutras)
            print('%#x %5s bytes, chama %d (%d indiretas), faltam: %s' % (a, n, len(alvos), ind, ' '.join('%x' % t for t in falta)))
        return
    if '--fora' in sys.argv:
        cobset = set(cob)
        fora = {}
        for a in cob:
            for t in chamadas(a)[0]:
                if t not in cobset and t not in feitas and t not in neutras and 0x6b3000 <= t < 0x718000:
                    fora.setdefault(t, set()).add(a)
        for t in sorted(fora, key=lambda t: tam.get(t, 0)):
            c = chamadas(t)[0] - feitas - neutras
            print('%#x %5d %-14s chamada por %d; chama %d nao feitas: %s' % (t, tam.get(t, 0), arq.get(t, '?'), len(fora[t]), len(c), ' '.join('%x' % x for x in sorted(c)[:5])))
        return
    saida = []
    for a in cob:
        if a in feitas or a not in tam:
            continue
        alvos, ind = chamadas(a)
        falta = sorted(t for t in alvos if t not in feitas and t not in neutras)
        saida.append((len(falta), tam[a], a, falta, ind))
    saida.sort()
    for nf, n, a, falta, ind in saida[:40]:
        print('%#x %5d %-16s falta %d %s ind=%d' % (a, n, arq.get(a, '?'), nf, ' '.join('%x' % f for f in falta[:6]), ind))
    print('restantes:', len(saida))


if __name__ == '__main__':
    main()
