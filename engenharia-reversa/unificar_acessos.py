r"""One memory dialect instead of four (10/11/2026, the outside reader's round 10, obstacle 2: "four memory dialects
coexist in one function"). The raw pointer forms become the unaligned accessors of motor/mem.rs (which are what every
`read_T`/`write_T` already is):

    *((E) as *const T)                       ->  read_T(E)
    core::ptr::read_unaligned((E) as *const T)  ->  read_T(E)
    *((E) as *mut T) = V;                    ->  write_T(E, V);
    core::ptr::write_unaligned((E) as *mut T, V) ->  write_T(E, V)

for T in the accessor types. Left alone: a compound assignment (`*(..) += v` would evaluate E twice), a deref taken by
reference (`&*`, `&mut *`), one followed by `.`/`[` (precedence), and a cast of something other than an address. x86
loads and stores the same way aligned or not, so the generated behaviour is the same; a function that does not compile
goes back, and the mutation snippets get the same rewrite.
Usage: python unificar_acessos.py [--gravar]
[pt] Um dialeto de memória só: as formas cruas de ponteiro viram os acessos read_T/write_T do mem.rs.
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

TIPOS = {'f64', 'i16', 'i32', 'i8', 'u16', 'u32', 'u8', 'usize', 'f32', 'i64', 'u64'}


def fecha(s, i):
    """Index after the parenthesis group that opens at s[i] == '(' (masked text), or None."""
    n = 0
    for k in range(i, len(s)):
        c = s[k]
        if c in '([{':
            n += 1
        elif c in ')]}':
            n -= 1
            if n == 0:
                return k + 1
    return None


def cast(s, a, b):
    """For the group s[a:b] == '(X as *const T)': (X text, const|mut, T) or None (top-level `as` only)."""
    dentro = s[a + 1:b - 1]
    m = re.search(r'\s+as\s+\*(const|mut)\s+(\w+)\s*$', dentro)
    if not m or m.group(2) not in TIPOS:
        return None
    x = dentro[:m.start()].strip()
    # the `as` must be at the top level of the group
    n = 0
    for c in x:
        n += c in '([{'
        n -= c in ')]}'
        if n < 0:
            return None
    if n != 0 or not x:
        return None
    if x.startswith('(') and fecha(x, 0) == len(x):
        x = x[1:-1].strip()
    return x, m.group(1), m.group(2)


def fim_da_instrucao(s, i):
    """Index of the `;` that ends the statement starting at i (top level), or None."""
    n = 0
    for k in range(i, len(s)):
        c = s[k]
        if c in '([{':
            n += 1
        elif c in ')]}':
            n -= 1
            if n < 0:
                return None
        elif c == ';' and n == 0:
            return k
    return None


def cast2(mk, s, a, b):
    """cast() validated on the masked text, with the expression taken from the real text at the same place."""
    c = cast(mk, a, b)
    if not c:
        return None
    r = cast(s, a, b)
    if not r or r[1:] != c[1:] or len(r[0]) != len(c[0]):
        return None
    return r


def converter(s):
    mk = tr.mascara(s)
    ed = []
    # core::ptr::read_unaligned((E) as *const T) / write_unaligned((E) as *mut T, V)
    for m in re.finditer(r'\b(?:core::ptr::)?(read|write)_unaligned\(', mk):
        a = m.end() - 1
        b = fecha(mk, a)
        if b is None:
            continue
        if m.group(1) == 'read':
            c = cast2(mk, s, a, b)
            if c and c[1] == 'const':
                ed.append((m.start(), b, 'read_%s(%s)' % (c[2], c[0])))
        else:
            # split at the top-level comma
            n, k0 = 0, None
            for k in range(a + 1, b - 1):
                ch = mk[k]
                n += ch in '([{'
                n -= ch in ')]}'
                if ch == ',' and n == 0:
                    k0 = k
                    break
            if k0 is None:
                continue
            gm, gs = '(' + mk[a + 1:k0] + ')', '(' + s[a + 1:k0] + ')'
            c = cast2(gm, gs, 0, len(gm))
            if c and c[1] == 'mut':
                ed.append((m.start(), b, 'write_%s(%s, %s)' % (c[2], c[0], s[k0 + 1:b - 1].strip())))
    # *((E) as *const T)  /  *((E) as *mut T) = V;
    for m in re.finditer(r'(?<![\w)\]])\*\(', mk):
        a = m.end() - 1
        b = fecha(mk, a)
        if b is None:
            continue
        c = cast2(mk, s, a, b)
        if not c:
            continue
        antes = mk[:m.start()].rstrip()
        if antes.endswith('&') or antes.endswith('&mut') or re.search(r'&\s*mut$', antes):
            continue
        depois = mk[b:b + 4]
        if re.match(r'\s*[.\[]', depois):
            continue
        mm = re.match(r'\s*(=)(?!=)', mk[b:])
        if mm:
            if c[1] != 'mut':
                continue
            v0 = b + mm.end()
            fim = fim_da_instrucao(mk, v0)
            if fim is None:
                continue
            ed.append((m.start(), fim, 'write_%s(%s, %s)' % (c[2], c[0], s[v0:fim].strip())))
        elif re.match(r'\s*[-+*/%&|^]=|\s*<<=|\s*>>=', mk[b:]):
            continue
        else:
            ed.append((m.start(), b, 'read_%s(%s)' % (c[2], c[0])))
    # only the outermost of overlapping edits (an inner one comes back on the next pass)
    ed.sort(key=lambda e: (e[0], -e[1]))
    livres, fim_ult = [], -1
    for e in ed:
        if e[0] >= fim_ult:
            livres.append(e)
            fim_ult = e[1]
    for a, b, txt in sorted(livres, reverse=True):
        s = s[:a] + txt + s[b:]
    return s, len(livres)


def converter_tudo(s):
    total = 0
    for _ in range(6):
        s, n = converter(s)
        total += n
        if not n:
            break
    return s, total


def main():
    grava = '--gravar' in sys.argv
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n, nf = {}, 0, 0
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
                novo, k = converter_tudo(bloco)
                if not k:
                    continue
                n += k
                nf += 1
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s: converter_tudo(s)[0]))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d acessos crus viram read/write em %d funções' % (n, nf))
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
