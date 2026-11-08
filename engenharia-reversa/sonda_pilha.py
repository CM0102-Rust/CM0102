r"""Acha os dwords de resíduo de pilha que mudam a carreira, pela sonda da DLL (rust/src/stack_probe.rs).

A rotina ENDERECO roda na pilha do jogo entre os trechos da sonda; a base é a carreira sem veneno (tem de ser a
referência: a sonda é transparente). Cada faixa [ini, fim) abaixo da entrada é envenenada a cada chamada; faixa
que muda a carreira é partida ao meio até chegar a dwords. Usa a DLL que está em jogo_testes (a compilada com a
sonda) e o modo original; as carreiras diferentes param no primeiro dia diferente (--referencia).

Uso: python sonda_pilha.py depois|dentro ENDERECO [--ate 2002-06-01] [--faixa 0-2000] [--veneno 5a5a5a5a]
"""
import argparse, os, re, subprocess, sys

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOG = os.path.join(RAIZ, 'jogo_testes', 'ddraw.log')
ap = argparse.ArgumentParser()
ap.add_argument('modo', choices=['depois', 'dentro'])
ap.add_argument('endereco')
ap.add_argument('--ate', default='2002-06-01')
ap.add_argument('--faixa', default='0-2000', help='bytes abaixo da entrada, hex')
ap.add_argument('--veneno', default='5a5a5a5a')
ap.add_argument('--menor', type=int, default=4, help='tamanho em bytes onde a bissecção para')
ap.add_argument('--dword', help='pula a bissecção da faixa: bissecta as chamadas deste dword (hex, bytes abaixo da entrada)')
args = ap.parse_args()
BASE_DIARIO = os.path.join(os.environ.get('TEMP', '.'), 'sonda_pilha_base.log')


def carreira(ini, fim, referencia=None, chamadas=(1, 9999999), mostrar=False):
    # larguras fixas: variáveis de ambiente de tamanhos diferentes deslocam a heap (simular.py)
    amb = dict(os.environ, CM_SONDA_PILHA='%s:%s' % (args.modo, args.endereco),
               CM_SONDA_FAIXA='%04x-%04x/%d-%d' % ((ini, fim) + chamadas), CM_SONDA_VENENO=args.veneno)
    if mostrar:
        amb['CM_SONDA_MOSTRAR'] = '1'
    cmd = [sys.executable, '-u', 'simular.py', '--ate', args.ate, '--motor', 'sim', '--modo', 'original', '--limite', '900']
    if referencia:
        cmd += ['--referencia', referencia]
    r = subprocess.run(cmd, cwd=os.path.dirname(os.path.abspath(__file__)), env=amb, capture_output=True, text=True)
    m = re.search(r'parou em ([^,]*),[^,]*, impressao (\w+)', r.stdout)
    return m.group(2)[:8] if m else 'saiu:' + (re.findall(r'execucao 1: (.*)', r.stdout) or [r.stdout[-200:]])[0][:60]


base = carreira(0, 0)
print('base (sem veneno):', base, '(referência be54a138)' if args.ate == '2002-06-01' else '', flush=True)
with open(LOG, encoding='utf-8', errors='replace') as f, open(BASE_DIARIO, 'w', encoding='utf-8') as g:
    g.writelines(l for l in f if l.startswith('dia '))
if args.dword:
    # a primeira chamada cujo resíduo naquele dword muda a carreira: menor b com as chamadas 1..b envenenadas mudando
    d = int(args.dword, 16)
    lo, hi = 0, 1
    while carreira(d, d + 4, BASE_DIARIO, (1, hi)) == base:
        print('chamadas 1..%d -> igual' % hi, flush=True)
        if hi == 9999999:
            sys.exit('nenhuma chamada muda a carreira nesta data')
        lo, hi = hi, min(hi * 4, 9999999)
    while hi - lo > 1:
        m = (lo + hi) // 2
        r = carreira(d, d + 4, BASE_DIARIO, (1, m))
        print('chamadas 1..%d -> %s' % (m, 'MUDA' if r != base else 'igual'), flush=True)
        lo, hi = (lo, m) if r != base else (m, hi)
    carreira(d, d + 4, BASE_DIARIO, (hi, hi), mostrar=True)
    print('CHAMADA %d; a cadeia:' % hi)
    print(''.join(l for l in open(LOG, encoding='utf-8', errors='replace') if 'sonda_pilha: chamada' in l))
    sys.exit()
ini, fim = (int(x, 16) for x in args.faixa.split('-'))
achados, fila = [], [(ini, fim)]
while fila:
    a, b = fila.pop(0)
    r = carreira(a, b, BASE_DIARIO)
    muda = r != base
    print('faixa %#06x..%#06x -> %s %s' % (a, b, r, 'MUDA' if muda else 'igual'), flush=True)
    if not muda:
        continue
    if b - a <= args.menor:
        achados.append(a)
        continue
    m = a + (b - a) // 2 // 4 * 4
    fila += [(a, m), (m, b)]
print('DWORDS LIDOS (bytes abaixo da entrada):', ' '.join('%#x' % x for x in achados) or 'nenhum')
