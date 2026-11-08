r"""Rodada 4 da legibilidade (etapa 4 da reorganização, 07/11/2026), pelos achados do leitor de fora, rodada 2:

1. tira as faixas de leva (`// ---------- Nª leva: ... ----------`) dos arquivos de rust/src/motor/ (a história fica no
   git, no wiki/log e em levas/N/);
2. dá uma linha em inglês aos comparadores (`compare_<x>`) que não têm (o comentário antigo fica como [pt]);
3. tamanho de registro com nome: `* 0x245` -> `* club::SIZE` (e pessoa 0x6e, nação 0x122), com SIZE em fields.rs;
4. mais globais com nome: um endereço de global de tipo não escalar (Date, objeto, ponteiro) em qualquer lugar do código
   (`dias(0xae2c90, ..)` -> `dias(G_CURRENT_DATE, ..)`); os escalares só onde o nome_globais.py já trocava.
O valor é o mesmo em tudo: o código gerado não muda. As listas de mutação recebem as mesmas trocas.

Uso: python arrumar_rodada4.py [--gravar]
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import trocar_nas_mutacoes  # noqa: E402
import nomear_globais  # noqa: E402

FAIXA = re.compile(r'^// ---------- .* ----------\n\n?', re.M)
PT = re.compile(r'[áéíóúãõçêôâà]| (?:de|que|não|com|para|pela|pelo|dos|das|uma|só)\b|ção|ções|ões', re.I)
TAMANHOS = {'0x245': 'club::SIZE', '0x6e': 'staff::SIZE', '0x122': 'nation::SIZE'}
MULT = re.compile(r'(\*\s*|wrapping_mul\()(0x245|0x6e|0x122)\b')
ESCALARES = {'s32', 'u32', 'u16', 's16', 'u8', 's8', 'float', 'double', 'bool', 'char', 'table', 'vtable', 'string', 'code'}


def arquivos():
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs') and f not in ('globals.rs', 'keys.rs', 'exe_fns.rs', 'fields.rs', 'mem.rs'):
                yield os.path.join(d, f)


def main():
    gravar = '--gravar' in sys.argv
    # as globais não escalares (Date, ponteiros, objetos): nome em qualquer lugar do código
    gl = {}
    p = os.path.join(R, 'referencias', 'cm0102-function-map', 'data-map', 'data.tsv')
    nomes = nomear_globais.mapa()
    for r in csv.DictReader(open(p, encoding='utf-8'), delimiter='\t'):
        a = int(r['address'], 16)
        if a in nomes and r['type'] not in ESCALARES and not r['type'].startswith('char'):
            gl[a] = nomes[a]
    usadas = {}
    LIT = re.compile(r'\b(0x[0-9a-f]{6})(usize)?\b')

    def globais(cod):
        def f(m):
            a = int(m.group(1), 16)
            if a in gl:
                usadas[a] = gl[a]
                return gl[a][0]
            return m.group(0)
        return LIT.sub(f, cod)

    def trocar_linha(l):
        cod, sep, com = l.partition('//')
        if cod.lstrip().startswith('///'):
            return l
        # 0x245 (581) só é o clube; 0x6e (110) e 0x122 (290) só nas linhas que falam de pessoa ou nação
        def tam(m):
            v = m.group(2)
            if v == '0x6e' and not re.search(r'pessoa|staff|STAFF|person', cod):
                return m.group(0)
            if v == '0x122' and not re.search(r'nac|nation|NATION', cod):
                return m.group(0)
            return m.group(1) + TAMANHOS[v]
        cod = MULT.sub(tam, cod)
        return globais(cod) + sep + com

    fase = sys.argv[sys.argv.index('--fase') + 1] if '--fase' in sys.argv else 'a'
    if fase == 'b':
        return fase_b(trocar_linha, usadas, gravar)
    faixas = comps = 0
    for p in arquivos():
        t = open(p, encoding='utf-8').read()
        n0 = len(FAIXA.findall(t))
        t2 = FAIXA.sub('', t)
        faixas += n0
        # comparadores sem linha em inglês
        partes, pos = [], 0
        for n, a, b in mover_rotinas.blocos(t2):
            if not n.startswith('compare_'):
                continue
            bloco = t2[a:b]
            corpo_ini = re.search(r'^(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn ', bloco, re.M).start()
            topo = bloco[:corpo_ini].split('\n')[:-1]
            docs = [l for l in topo if l.lstrip().startswith('///')]
            attrs = [l for l in topo if l.lstrip().startswith('#[')]
            if docs and (not PT.search(docs[0]) or any('[pt]' in l for l in docs)):
                continue
            alvo = n[len('compare_'):]
            cab = ['/// Comparer for `%s`.' % alvo]
            if docs:
                docs[0] = docs[0].replace('/// ', '/// [pt] ', 1)
                cab += ['///'] + docs
            partes.append(t2[pos:a])
            partes.append('\n'.join(cab + attrs) + '\n' + bloco[corpo_ini:])
            pos = b
            comps += 1
        partes.append(t2[pos:])
        t3 = ''.join(partes)
        if t3 != t and gravar:
            open(p, 'w', encoding='utf-8').write(t3)
    print('fase a: %d faixas de leva saem; %d comparadores ganham a linha em inglês' % (faixas, comps))


def fase_b(trocar_linha, usadas, gravar):
    """Os tamanhos e as globais não escalares: a linha que não compilar com o nome volta ao texto de antes (o nome só
    entra onde cabe sem conversão de tipo)."""
    import subprocess
    antes = {p: open(p, encoding='utf-8').read() for p in arquivos()}
    for p, t in antes.items():
        novo = '\n'.join(trocar_linha(l) for l in t.split('\n'))
        if novo != t and gravar:
            open(p, 'w', encoding='utf-8').write(novo)
    print('fase b: %d globais não escalares com nome' % len(usadas))
    if not gravar:
        return
    gerados(usadas)
    for volta in range(10):
        r = subprocess.run(['cargo', 'build', '--release', '--message-format=short'], cwd=os.path.join(R, 'rust'),
                           capture_output=True, text=True, encoding='utf-8', errors='replace')
        erros = re.findall(r'^(src[^:]+):(\d+):\d+: error', r.stderr, re.M)
        if not erros:
            break
        por = {}
        for f, l in erros:
            por.setdefault(os.path.normpath(os.path.join(R, 'rust', f)), set()).add(int(l))
        n = 0
        for p, linhas in por.items():
            if p not in antes:
                continue
            ls, orig = open(p, encoding='utf-8').read().split('\n'), antes[p].split('\n')
            for l in linhas:
                if ls[l - 1] != orig[l - 1]:
                    ls[l - 1] = orig[l - 1]
                    n += 1
            open(p, 'w', encoding='utf-8').write('\n'.join(ls))
        print('volta %d: %d erros, %d linhas voltaram ao texto de antes' % (volta, len(erros), n))
        if n == 0:
            sys.exit('erros que não vêm desta troca:\n' + r.stderr[-2000:])
    # as listas de mutação: cada trecho fica com a versão (com ou sem a troca) que existe uma vez só no código
    import runpy
    from fila import texto_do_motor
    t = texto_do_motor(R)
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in os.listdir(pasta):
        if not f.endswith('.py'):
            continue
        p = os.path.join(pasta, f)
        try:
            ms = runpy.run_path(p).get('MUTACOES', [])
        except Exception:
            continue
        texto = open(p, encoding='utf-8').read()
        mudou = False
        for m in ms:
            nt = '\n'.join(trocar_linha(l) for l in m[1].split('\n'))
            if nt != m[1] and t.count(m[1]) != 1 and t.count(nt) == 1:
                nn = '\n'.join(trocar_linha(l) for l in m[2].split('\n'))
                texto = texto.replace(repr(m[1]), repr(nt), 1).replace(repr(m[2]), repr(nn), 1)
                mudou = True
        if mudou:
            open(p, 'w', encoding='utf-8').write(texto)
    print('gravado')


def gerados(usadas):
    # as globais novas e os tamanhos nos arquivos gerados
    pg = os.path.join(SRC, 'motor', 'globals.rs')
    g = open(pg, encoding='utf-8').read()
    novas = ''.join('/// %s (%s)\npub(crate) const %s: usize = %#x;\n' % (d or n, t, n, a) for a, (n, t, d) in sorted(usadas.items())
                    if 'const %s:' % n not in g)
    open(pg, 'w', encoding='utf-8').write(g.rstrip('\n') + '\n' + novas)
    pf = os.path.join(SRC, 'motor', 'fields.rs')
    f = open(pf, encoding='utf-8').read()
    for mod, tam in (('club', 0x245), ('staff', 0x6e), ('nation', 0x122)):
        if 'pub(crate) mod %s {' % mod in f and 'const SIZE' not in f.split('pub(crate) mod %s {' % mod)[1].split('}')[0]:
            f = f.replace('pub(crate) mod %s {\n' % mod, 'pub(crate) mod %s {\n    pub(crate) const SIZE: usize = %#x; // the record size (%d bytes)\n' % (mod, tam, tam), 1)
    open(pf, 'w', encoding='utf-8').write(f)
    print('gravado')


if __name__ == '__main__':
    main()
