r"""Raw offsets on typed records get their field's name (10/11/2026, the outside reader's round 10, obstacle 1: "the
same field three ways within 15 lines": `+ 0x27`, `+ match_player::SIDE`, `p.side()`). In a function, `x` is typed when
it is a parameter `x: Club` or bound `let x = Club(..)`; a chain `x.club_job()` takes the getter's return type
(ponteiros_registros.py). Where such an expression is followed by `.addr() + 0xNN` and the record's module in fields.rs
has exactly one field at 0xNN, the literal becomes `club::FIELD`. The access itself stays raw: `tipar_registros.py
--acessos MODULE` then turns it into a method. Offsets with no field (or two) stay. The mutation lists get the same
change, snippet by snippet (`seguir_ancoras.py` covers what is left).
Usage: python nomear_deslocamentos.py [--gravar]
[pt] Deslocamentos crus sobre registros tipados ganham o nome do campo quando o módulo tem um único campo nele.
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import ponteiros_registros as pr  # noqa: E402
import tipar_registros as tr  # noqa: E402


def campos_por_desloc():
    """{module: {offset: FIELD}} with only offsets held by exactly one field (sizes and strides left out)."""
    t = open(os.path.join(SRC, 'motor', 'fields.rs'), encoding='utf-8').read()
    out = {}
    for m in re.finditer(r'pub\(crate\) mod (\w+) \{(.*?)\n\}', t, re.S):
        por = collections.defaultdict(list)
        for c in re.finditer(r'pub\(crate\) const (\w+): usize = (0x[0-9a-f]+|\d+);', m.group(2)):
            nome = c.group(1)
            if nome == 'SIZE' or nome.endswith('_SIZE') or nome.endswith('_STRIDE') or nome.endswith('_COUNT_MAX'):
                continue
            por[int(c.group(2), 0)].append(nome)
        out[m.group(1)] = {o: ns[0] for o, ns in por.items() if len(ns) == 1}
    return out


def tipos_modulos():
    t = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    return {m.group(2): m.group(1) for m in re.finditer(r'/// A `(\w+)` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(', t)}


def main():
    grava = '--gravar' in sys.argv
    campos = campos_por_desloc()
    mod_de = tipos_modulos()
    _, ret_getter, _ = pr.registros()
    nomes_t = '|'.join(sorted(mod_de, key=len, reverse=True))
    mutacoes = tr.carregar_mutacoes()
    trocas_mut = {}
    n, por_mod = 0, collections.Counter()
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                mk = tr.mascara(bloco)
                tipadas = {}
                for m in re.finditer(r'(?<![\w.])([a-z_][a-z0-9_]*)\s*:\s*(%s)\b' % nomes_t, mk):
                    tipadas[m.group(1)] = m.group(2)
                for m in re.finditer(r'\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*=\s*(%s)\(' % nomes_t, mk):
                    tipadas[m.group(1)] = m.group(2)
                if not tipadas:
                    continue
                ed = []
                for m in re.finditer(r'(?<![\w.])([a-z_][a-z0-9_]*)((?:\.[a-z_0-9]+\(\))*)\.addr\(\)\s*\+\s*(0x[0-9a-f]+)\b(?!\s*[*/])', mk):
                    tp = tipadas.get(m.group(1))
                    for meth in re.findall(r'\.([a-z_0-9]+)\(\)', m.group(2)):
                        tp = ret_getter.get((tp, meth)) if tp else None
                    if not tp or tp not in mod_de:
                        continue
                    mod = mod_de[tp]
                    campo = campos.get(mod, {}).get(int(m.group(3), 16))
                    if not campo:
                        continue
                    ed.append((m.start(3), m.end(3), '%s::%s' % (mod, campo)))
                    por_mod[mod] += 1
                if not ed:
                    continue
                novo = bloco
                for a2, b2, txt in sorted(ed, reverse=True):
                    novo = novo[:a2] + txt + novo[b2:]
                n += len(ed)
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        # the same replacements, by text, inside the snippet and its replacement
                        def conv(s, ed=ed, bloco=bloco):
                            for a2, b2, txt in ed:
                                ctx = bloco[max(0, a2 - 40):b2]
                                if ctx in s:
                                    s = s.replace(ctx, ctx[:len(ctx) - (b2 - a2)] + txt)
                            return s
                        trocas_mut[(lista, mu[0])] = (mu, conv)
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d deslocamentos nomeados' % n, dict(por_mod.most_common(15)))
    if grava:
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
