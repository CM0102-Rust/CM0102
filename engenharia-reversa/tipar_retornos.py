r"""Our functions that return a record as a bare `usize` return the record type (10/11/2026, typed records: "functions
returning a club as usize": `all_star_club(g) + 0xd7`). A non-`extern` function `fn f(..) -> usize` is voted by how its
callers use the value: wrapped in a record type (`Club(f(..))`) or as the base of a record's field
(`f(..) + club::SQUAD`). One type with at least 2 votes and 80% of them: the signature becomes `-> Club` and the
compiler fixes the rest (`tipar_registros.py --ajustar`: the returned expressions get `Club(..)`, the callers that want
an address get `.addr()`); then `Club(f(..).addr())` becomes `f(..)`. Functions used as values (callbacks, tables) stay.
Usage: python tipar_retornos.py [--gravar]
[pt] As nossas funções que devolvem um registro em `usize` passam a devolver o tipo, pelo uso que as chamadas fazem.
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402

EXCLUIR = ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs')


def fontes():
    out = {}
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if f.endswith('.rs') and f not in EXCLUIR:
                p = os.path.join(raiz, f)
                out[p] = open(p, encoding='utf-8').read()
    return out


def main():
    grava = '--gravar' in sys.argv
    rec = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    mod_tipo = {m.group(1): m.group(2) for m in re.finditer(r'/// A `(\w+)` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(', rec)}
    tipos = '|'.join(sorted(mod_tipo.values(), key=len, reverse=True))
    fs = fontes()
    tudo = '\n'.join(tr.mascara(t) for t in fs.values())
    # candidate functions: `fn name(..) -> usize`, not extern, not used as a value
    cand = {}
    for p, t in fs.items():
        mk = tr.mascara(t)
        for m in re.finditer(r'^[ \t]*(?:pub\(crate\) )?(?:unsafe )?fn (\w+)\(', mk, re.M):
            args, fim = ie.args_da_chamada(mk, m.end() - 1)
            if re.match(r'\s*->\s*usize\s*(?:\{|where)', mk[fim:fim + 40]):
                cand[m.group(1)] = p
    soltos = {m.group(1) for m in re.finditer(r'(?<![\w.:])(?<!fn )([a-z_][a-z0-9_]*)\b(?!\s*\()(?!\s*::)', tudo)}
    votos = collections.defaultdict(collections.Counter)
    for m in re.finditer(r'\b(%s)\(\s*(\w+)\(' % tipos, tudo):
        if m.group(2) in cand:
            args, fim = ie.args_da_chamada(tudo, m.end() - 1)
            if fim is not None and re.match(r'\s*\)', tudo[fim:fim + 3]):
                votos[m.group(2)][m.group(1)] += 1
    for m in re.finditer(r'(?<![\w.:])(\w+)\(', tudo):
        if m.group(1) not in cand:
            continue
        args, fim = ie.args_da_chamada(tudo, m.end() - 1)
        if fim is None:
            continue
        mm = re.match(r'\s*\+\s*([a-z_0-9]+)::[A-Z]', tudo[fim:fim + 60])
        if mm and mm.group(1) in mod_tipo:
            votos[m.group(1)][mod_tipo[mm.group(1)]] += 1
    escolha = {}
    for nome, c in votos.items():
        if nome in soltos:
            continue
        tipo, n = c.most_common(1)[0]
        if n >= 2 and n * 5 >= sum(c.values()) * 4:
            escolha[nome] = tipo
    print('%d funções passam a devolver um registro' % len(escolha))
    for nome, tipo in sorted(escolha.items()):
        print('  %-45s -> %s  %s' % (nome, tipo, dict(votos[nome])))
    if not grava or not escolha:
        return
    por_arq = collections.defaultdict(list)
    for nome in escolha:
        por_arq[cand[nome]].append(nome)
    for p, nomes in por_arq.items():
        t = fs[p]
        for nome in nomes:
            t = re.sub(r'(fn %s\((?:[^()]|\([^()]*\))*\)\s*->\s*)usize\b' % nome, r'\g<1>%s' % escolha[nome], t, count=1)
        open(p, 'w', encoding='utf-8').write(t)
    usados = sorted(set(escolha.values()))
    tr.ajustar_pelo_compilador(usados)
    # Club(f(..).addr()) -> f(..)
    for p in fontes():
        t = open(p, encoding='utf-8').read()
        novo = t
        for nome, tipo in escolha.items():
            novo = re.sub(r'\b%s\((%s\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\))\.addr\(\)\)' % (tipo, nome), r'\1', novo)
        if novo != t:
            open(p, 'w', encoding='utf-8').write(novo)
    tr.ajustar_pelo_compilador(usados)


if __name__ == '__main__':
    main()
