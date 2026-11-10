r"""Inline transmutes that duplicate an exe:: wrapper become the wrapper (10/11/2026, the outside reader's round 10,
obstacle 6: `contract_manager.rs:180` transmutes FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS though exe::scout_manager_sync_scout_lists
exists). In a function, `let NAME: unsafe extern "CC" fn(ARGS) -> RET = core::mem::transmute(FN_X);` is removed and
every call `NAME(..)` becomes `exe::wrapper(..)` when exe_calls.rs has a wrapper for FN_X with the SAME calling convention,
the same number of arguments and the same return type (a different one would change what is passed or read). A
local used other than by calling it (passed on as a value) stays. A function that does not compile goes back to its
previous text (an argument type the wrapper takes differently); the mutation snippets follow.
Usage: python usar_involucros.py [--gravar]
[pt] Os transmutes inline que repetem um invólucro exe:: viram o invólucro (mesma convenção, aridade e retorno).
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402
import traduzir_locais as tl  # noqa: E402

LET = re.compile(r'^([ \t]*)let (\w+): unsafe extern "(\w+)" fn\(([^()]*)\)(?:\s*->\s*([\w*]+(?: \w+)?))?\s*=\s*core::mem::transmute\((FN_\w+)\);[ \t]*\n', re.M)


def aridade(args):
    args = args.strip()
    return 0 if not args else len(ie.args_da_chamada('(' + args + ')', 0)[0] or [])


def involucros():
    """{FN_X: (wrapper, convention, arity, return)} from exe_calls.rs."""
    t = open(os.path.join(SRC, 'motor', 'exe_calls.rs'), encoding='utf-8').read()
    out = {}
    for m in re.finditer(r'pub\(crate\) unsafe fn (\w+)\([^)]*\)[^{]*\{\s*\n\s*let f: unsafe extern "(\w+)" fn\(([^()]*)\)(?:\s*->\s*([\w*]+(?: \w+)?))?\s*=\s*core::mem::transmute\((FN_\w+)\);', t):
        out.setdefault(m.group(5), (m.group(1), m.group(2), aridade(m.group(3)), (m.group(4) or '').strip()))
    return out


def main():
    grava = '--gravar' in sys.argv
    inv = involucros()
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n, nf = {}, 0, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('exe_calls.rs', 'records.rs', 'fields.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]

                def conv(s):
                    k = 0
                    for m in list(LET.finditer(s))[::-1]:
                        nome, cc, args, ret, fn = m.group(2), m.group(3), m.group(4), (m.group(5) or '').strip(), m.group(6)
                        w = inv.get(fn)
                        if not w or w[1] != cc or w[2] != aridade(args) or w[3] != ret:
                            continue
                        resto = s[:m.start()] + s[m.end():]
                        mk = tr.mascara(resto)
                        usos = [x for x in re.finditer(r'(?<![\w.:])%s\b' % re.escape(nome), mk)]
                        if not usos or any(not re.match(r'\s*\(', mk[x.end():x.end() + 3]) for x in usos):
                            continue  # not used, or used as a value
                        if re.search(r'\blet\s+(?:mut\s+)?%s\b|\|[^|]*\b%s\b[^|]*\|' % (re.escape(nome), re.escape(nome)), mk):
                            continue  # shadowed by another binding
                        for x in usos[::-1]:
                            resto = resto[:x.start()] + 'exe::' + w[0] + resto[x.end():]
                        s = resto
                        k += 1
                    return s, k

                novo, k = conv(bloco)
                if not k:
                    continue
                n += k
                nf += 1
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s: conv(s)[0]))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d transmutes viram invólucros em %d funções' % (n, nf))
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
