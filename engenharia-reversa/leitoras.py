r"""Varredura das leitoras de resíduo de pilha: as rotinas originais que leem variável própria antes de gravar.

Cada rotina candidata (do docs/mapa_funcoes.csv, sem desvio próprio: fora do motor.rs e do data.rs, ou nas DESLIGADAS)
ganha na DLL (CM_SONDA_LISTA=1, rust/src/sonda_pilha.rs) um desvio transparente que, se a rotina estiver marcada,
envenena o próprio quadro na entrada ([entrada − quadro, entrada)). Carreira diferente da base = alguma marcada lê
variável sem valor. A busca é em grupos: o conjunto que muda é partido ao meio até sobrar a rotina.

    python leitoras.py lista                 gera jogo_testes\sonda_lista.txt (todas desmarcadas) e o resumo
    python leitoras.py varrer [--ate DATA]   a busca em grupos; o andamento vai para levas\leitoras\varredura.txt

O quadro sai do prólogo (pushes, sub esp, __chkstk), contado até o primeiro salto, chamada ou retorno.
"""
import argparse, bisect, csv, os, re, subprocess, sys
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LISTA = os.path.join(RAIZ, 'jogo_testes', 'sonda_lista.txt')
LOG = os.path.join(RAIZ, 'jogo_testes', 'ddraw.log')
SAIDA = os.path.join(RAIZ, 'levas', 'leitoras')
EXE = open(os.path.join(RAIZ, 'jogo', 'cm0102-1080.exe'), 'rb').read()
md = Cs(CS_ARCH_X86, CS_MODE_32)
md.detail = False


def ja_desviadas():
    src = open(os.path.join(RAIZ, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    src_d = open(os.path.join(RAIZ, 'rust', 'src', 'data.rs'), encoding='utf-8').read()
    desl = re.search(r'const DESLIGADAS: &\[usize\] = &\[(.*?)\];', src).group(1)
    desligadas = {int(x, 16) for x in re.findall(r'0x[0-9a-f]+', desl)}
    # só as tabelas instaladas no modo original (as portas da comparação, PROLOGOS_POS e PROLOGO_*, ficam fora)
    blocos = [re.search(r'const %s: \[.*?\n\];' % t, src, re.S).group(0) for t in ('PROLOGOS', 'ZERADAS')]
    todas = {int(x, 16) for x in re.findall(r'endereco: (0x[0-9a-f]+)', ''.join(blocos) + src_d)}
    # o sorteio e a data têm desvio próprio fora das tabelas
    todas |= {0x90cfc0}
    return todas - desligadas


def prologo(a):
    """(bytes a copiar, quadro) ou None: instruções inteiras, sem salto relativo nos 5 primeiros bytes."""
    o = a - 0x400000
    ins = list(md.disasm(EXE[o:o + 96], a))
    n, copiar = 0, 0
    for i in ins:
        if copiar >= 5:
            break
        if i.mnemonic.startswith('j') or i.mnemonic in ('call', 'ret', 'loop', 'jecxz', 'int3', 'retn') or 'eip' in i.op_str:
            return None
        copiar += i.size
    if copiar < 5:
        return None
    quadro, mov_eax = 0, 0
    for i in ins[:40]:
        m, op = i.mnemonic, i.op_str
        if m == 'push':
            quadro += 4
        elif m == 'sub' and op.startswith('esp, '):
            quadro += int(op.split(', ')[1], 0)
        elif m == 'mov' and op.startswith('eax, 0x') and ',' in op and '[' not in op:
            mov_eax = int(op.split(', ')[1], 0)
        elif m == 'call':
            if mov_eax >= 0x1000:  # __chkstk: o eax é o tamanho do quadro
                quadro += mov_eax
            break
        elif m.startswith('j') or m.startswith('ret') or m == 'pop' or (m == 'add' and op.startswith('esp')):
            break
    return copiar, quadro


def gerar_lista():
    feitas = ja_desviadas()
    linhas, recusadas = [], 0
    for r in csv.DictReader(open(os.path.join(RAIZ, 'docs', 'mapa_funcoes.csv'), encoding='utf-8')):
        a = int(r['endereco'], 16)
        if a in feitas or not r['arquivo'].startswith('cm3/'):
            continue  # só o código do jogo (fora a CRT e as bibliotecas)
        p = prologo(a)
        if not p or p[1] == 0:
            recusadas += 1
            continue
        linhas.append((a, p[0], p[1]))
    os.makedirs(SAIDA, exist_ok=True)
    return linhas, recusadas


def gravar(linhas, marcadas):
    with open(LISTA, 'w') as f:
        for a, n, q in linhas:
            f.write('%x %d %x %d\n' % (a, n, q, 1 if a in marcadas else 0))


def carreira(ate, referencia=None):
    amb = dict(os.environ, CM_SONDA_LISTA='1')
    cmd = [sys.executable, '-u', 'simular.py', '--ate', ate, '--motor', 'sim', '--modo', 'original', '--limite', '400']
    if referencia:
        cmd += ['--referencia', referencia]
    r = subprocess.run(cmd, cwd=os.path.dirname(os.path.abspath(__file__)), env=amb, capture_output=True, text=True)
    m = re.search(r'parou em ([^,]*),[^,]*, impressao (\w+)', r.stdout)
    return m.group(2)[:8] if m else 'saiu:' + (re.findall(r'execucao 1: (.*)', r.stdout) or [r.stdout[-200:]])[0][:70]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('acao', choices=['lista', 'varrer'])
    ap.add_argument('--ate', default='2002-06-01')
    ap.add_argument('--de', type=int, default=0, help='começa a varredura nesta posição da lista (retomar)')
    args = ap.parse_args()
    linhas, recusadas = gerar_lista()
    print('%d rotinas candidatas, %d recusadas (prólogo com salto relativo ou sem quadro)' % (len(linhas), recusadas))
    if args.acao == 'lista':
        gravar(linhas, set())
        return
    gravar(linhas, set())
    base = carreira(args.ate)
    diario = os.path.join(SAIDA, 'base_diario.txt')
    with open(LOG, encoding='utf-8', errors='replace') as f, open(diario, 'w', encoding='utf-8') as g:
        g.writelines(l for l in f if l.startswith('dia '))
    rel = open(os.path.join(SAIDA, 'varredura.txt'), 'a', encoding='utf-8')

    def nota(t):
        print(t, flush=True)
        rel.write(t + '\n')
        rel.flush()

    nota('base %s (lista transparente, %d rotinas, até %s)' % (base, len(linhas), args.ate))
    enderecos = [a for a, _, _ in linhas][args.de:]
    leitoras, fila = [], [enderecos]
    while fila:
        g = fila.pop(0)
        gravar(linhas, set(g))
        r = carreira(args.ate, diario)
        muda = r != base
        nota('%5d rotinas %#x..%#x -> %s %s' % (len(g), g[0], g[-1], r, 'MUDA' if muda else 'igual'))
        if not muda:
            continue
        if len(g) == 1:
            leitoras.append(g[0])
            nota('LEITORA %#x' % g[0])
            continue
        m = len(g) // 2
        fila = [g[:m], g[m:]] + fila  # em profundidade: cada leitora sai logo
    nota('LEITORAS: ' + ' '.join('%#x' % a for a in leitoras))


if __name__ == '__main__':
    main()
