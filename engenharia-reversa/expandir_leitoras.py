r"""Local reader closures over fixed exe addresses are expanded (10/11/2026, readability): in a function with
`let k = |a: usize| read_f64(a);` (or `read_f32(a)`, `read_f32(a) as f64`), each call with a literal address
`k(0x968dc0)` becomes `read_f64(0x968dc0)`, so nomear_rdata.py can then name the constant by its value
(`rdata::F64_0_5`); the closure line goes when no other use is left. Calls with a computed address stay.
Usage: python expandir_leitoras.py [--gravar]
[pt] Os fechos locais que só leem um endereço fixo do exe são expandidos, para o nomear_rdata.py dar nome à constante.
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

DEF = re.compile(r'^([ \t]*)let (\w+) = \|(\w+): usize\| (read_f(?:32|64))\(\3\)( as f64)?;[ \t]*\n', re.M)


def converter(bloco):
    n = 0
    for m in list(DEF.finditer(bloco))[::-1]:
        nome, leitor, conv = m.group(2), m.group(4), m.group(5) or ''
        corpo = bloco[m.end():]
        novo, k = re.subn(r'(?<![\w.])%s\((0x[0-9a-f]{6})\)' % re.escape(nome),
                          (lambda x: '%s(%s)%s' % (leitor, x.group(1), conv) if not conv else '(%s(%s)%s)' % (leitor, x.group(1), conv)), corpo)
        if not k:
            continue
        resto = re.search(r'(?<![\w.])%s\b' % re.escape(nome), tr.mascara(novo))
        bloco = bloco[:m.start()] + (bloco[m.start():m.end()] if resto else '') + novo
        n += k
    return bloco, n


def main():
    grava = '--gravar' in sys.argv
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n = {}, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs'):
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
                defs = {(m.group(2), m.group(4), m.group(5) or '') for m in DEF.finditer(bloco)}

                def conv_trecho(s, defs=defs):
                    for nome, leitor, c in defs:
                        s = re.sub(r'(?<![\w.])%s\((0x[0-9a-f]{6})\)' % re.escape(nome),
                                   (lambda x: '%s(%s)' % (leitor, x.group(1)) if not c else '(%s(%s)%s)' % (leitor, x.group(1), c)), s)
                    return s
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, conv_trecho)
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d chamadas de fechos leitores expandidas' % n)
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
