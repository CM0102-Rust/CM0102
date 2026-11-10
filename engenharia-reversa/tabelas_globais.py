r"""The game's club and person tables by name (10/11/2026, the outside reader's round 10: "`club += 0x245` (club::SIZE
exists)"). The inline arithmetic on the global tables becomes the two accessors of motor/mem.rs:

    read_usize(G_CLUB_TABLE).wrapping_add((i as isize * 0x245) as usize)   ->  club_at(i as isize).addr()
    (read_usize(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize     ->  staff_at(id as isize).addr()

(and the same with the table read as `read_u32(..) as usize`, the index first or second, `wrapping_add` or `+`); a
record pointer stepped through a table (`club += 0x245`, `p += 0x6e` where p walks G_STAFF_TABLE) takes the named size.
The accessors wrap like the original's 32-bit arithmetic and check no bounds, as the inline code did. Where the old
expression was wrapped in `Club(..)`/`Staff(..)` the wrapper goes; elsewhere `.addr()` keeps the `usize`.
Usage: python tabelas_globais.py [--gravar]
[pt] As tabelas de clubes e de pessoas do jogo com nome: club_at(i) e staff_at(id) no lugar da conta à mão.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402
import traduzir_locais as tl  # noqa: E402

PAR = r'\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\)'
IDX = r'(?:[\w.]+(?:%s)?|%s)(?: as \w+)*' % (PAR, PAR)
TAB = {'CLUB': ('0x245', 'club_at', 'Club'), 'STAFF': ('0x6e', 'staff_at', 'Staff')}


def formas(nome):
    tam = TAB[nome][0]
    base = r'(?:read_usize\(G_%s_TABLE\)|read_u32\(G_%s_TABLE\) as usize|\(read_u32\(G_%s_TABLE\) as usize\))' % (nome, nome, nome)
    base_i = r'(?:read_usize\(G_%s_TABLE\) as isize|read_u32\(G_%s_TABLE\) as isize|read_i32\(G_%s_TABLE\) as isize)' % (nome, nome, nome)
    return [
        # base.wrapping_add((I as isize * SZ) as usize)
        base + r'\.wrapping_add\(\((' + IDX + r') \* ' + tam + r'\) as usize\)',
        base + r'\.wrapping_add\(\(' + tam + r' \* (' + IDX + r')\) as usize\)',
        # (base as isize + I * SZ) as usize
        r'\(' + base_i + r' \+ (' + IDX + r') \* ' + tam + r'\) as usize',
        r'\(' + base_i + r' \+ ' + tam + r' \* (' + IDX + r')\) as usize',
        # base + SZ * I   /   base + I * SZ
        base + r' \+ ' + tam + r' \* (' + IDX + r')(?![\w.(\[])',
        base + r' \+ (' + IDX + r') \* ' + tam + r'(?![\w.(\[])',
    ]


def sem_cast(x):
    x = x.strip()
    while True:
        m = re.fullmatch(r'(.*\S) as (?:isize|usize|i32|u32)', x, re.S)
        if not m:
            return x
        x = m.group(1)


def converter(s):
    n = 0
    for nome, (tam, fn, tipo) in TAB.items():
        for rx in formas(nome):
            def f(m):
                i = sem_cast(m.group(1))
                if not re.fullmatch(r'[\w.:]+(?:%s)?' % PAR, i):
                    i = '(%s)' % i
                return '%s(%s as isize).addr()' % (fn, i)
            s, c = re.subn(rx, f, s)
            n += c
        # Club(club_at(i).addr()) -> club_at(i)
        s = re.sub(r'\b%s\((%s\(%s\))\.addr\(\)\)' % (tipo, fn, r'(?:[^()]|\((?:[^()]|\([^()]*\))*\))*'), r'\1', s)
    # a pointer stepped through the club table
    s, c = re.subn(r'\b(club|c|p|clube)\s*\+=\s*0x245;', r'\1 += club::SIZE;', s)
    n += c
    return s, n


def main():
    grava = '--gravar' in sys.argv
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n = {}, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('exe_calls.rs', 'records.rs', 'fields.rs', 'mem.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                novo, k = converter(bloco)
                if not k:
                    continue
                n += k
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s: converter(s)[0]))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d contas de tabela viram club_at/staff_at/SIZE' % n)
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
