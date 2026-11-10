r"""Cleanup after the typed-records migration (09/11/2026, batch 14b): the compiler-driven fixes leave noise around typed
values. Three rewrites:

- `(x.f()).addr()` -> `x.f().addr()` (redundant parentheses);
- `x == Club(0)` / `Club(0) == x` -> `x.is_null()`, `!=` -> `!x.is_null()` (the same test);
- `a.addr() == b.addr()` -> `a == b` (and `!=`): only when both sides are the same record type, which the compiler
  decides: a function that does not compile goes back to its previous text (traduzir_locais).

The first two are plain text equivalences and are also applied to the mutation lists; the third changes code only (the
mutation anchors are then followed with seguir_ancoras.py).
Usage: python limpar_tipos.py [--gravar]
[pt] Limpeza depois da migração para os registros tipados: parênteses redundantes, teste de nulo e comparação entre
tipados sem `.addr()`.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import traduzir_locais as tl  # noqa: E402

_REC = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
TIPOS = r'(?:%s)' % '|'.join(sorted(re.findall(r'pub\(crate\) struct (\w+)\(pub\(crate\) usize\);', _REC), key=len, reverse=True))
CADEIA = r'[a-z_][a-z0-9_]*(?:\(\))?(?:\.[a-z_0-9]+\(\))*'


def texto(s):
    """The plain-text equivalences (doubled parentheses around a chain go first, until nothing changes)."""
    while True:
        s2 = re.sub(r'\(\((%s)\)\)' % CADEIA, r'(\1)', s)
        s2 = re.sub(r'\((%s)\)\.addr\(\)' % CADEIA, r'\1.addr()', s2)
        if s2 == s:
            break
        s = s2
    s = re.sub(r'(?<![\w.])(%s)\s*==\s*%s\(0\)' % (CADEIA, TIPOS), r'\1.is_null()', s)
    s = re.sub(r'(?<![\w.])(%s)\s*!=\s*%s\(0\)' % (CADEIA, TIPOS), r'!\1.is_null()', s)
    s = re.sub(r'(?<![\w.])%s\(0\)\s*==\s*(%s)\b(?!\()' % (TIPOS, CADEIA), r'\1.is_null()', s)
    s = re.sub(r'(?<![\w.])%s\(0\)\s*!=\s*(%s)\b(?!\()' % (TIPOS, CADEIA), r'!\1.is_null()', s)
    return s


def comparacoes(s):
    """Compiler-checked (a function that does not compile goes back): `a.addr() == b.addr()` -> `a == b`, and a
    rewrap of the same type `Club(x.club().addr())` -> `x.club()`."""
    s = re.sub(r'(?<![\w.])(%s)\.addr\(\)\s*(==|!=)\s*(%s)\.addr\(\)(?![\w.(])' % (CADEIA, CADEIA), r'\1 \2 \3', s)
    return re.sub(r'(?<![\w.:])%s\((%s)\.addr\(\)\)' % (TIPOS, CADEIA), r'\1', s)


def main():
    grava = '--gravar' in sys.argv
    n1 = n2 = 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            p = os.path.join(raiz, f)
            if not f.endswith('.rs') or f in ('records.rs', 'exe_calls.rs'):
                continue
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            t1 = texto(t)
            n1 += t1 != t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t1):
                bl = t1[a:b]
                nv = comparacoes(bl)
                if nv != bl:
                    n2 += 1
                    partes += [t1[pos:a], nv]
                    pos = b
            partes.append(t1[pos:])
            novo = ''.join(partes)
            if grava and novo != t:
                open(p, 'w', encoding='utf-8').write(novo)
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    nm = 0
    for f in os.listdir(pasta):
        if f.endswith('.py'):
            p = os.path.join(pasta, f)
            t = open(p, encoding='utf-8').read()
            t1 = texto(t)
            if t1 != t:
                nm += 1
                if grava:
                    open(p, 'w', encoding='utf-8').write(t1)
    print('%d arquivos com trocas de texto, %d funções com comparações, %d listas de mutação' % (n1, n2, nm))
    if grava:
        tl.voltar_funcoes_com_erro({})


if __name__ == '__main__':
    main()
