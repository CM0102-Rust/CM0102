r"""Follows the mutation anchors broken by a mechanical change of the Rust sources (09/11/2026): for each mutation whose
anchor was found once in the sources at a commit and is no longer found, the lines it covered are matched to the
changed lines (difflib) and the same textual change is applied to the anchor and to the replacement (the parts of the
line around the anchor must be unchanged). Prints what it could not follow.
Usage: python seguir_ancoras.py COMMIT [--gravar]
[pt] Segue as âncoras de mutação quebradas por uma troca mecânica: acha a linha antiga e a nova (difflib) e aplica a
mesma troca na âncora e no texto de troca.
"""
import difflib
import os
import re
import runpy
import subprocess
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PASTA = os.path.join(R, 'engenharia-reversa', 'mutacoes')


def fontes_em(commit):
    nomes = subprocess.run(['git', 'ls-tree', '-r', '--name-only', commit, 'rust/src'], cwd=R, capture_output=True, text=True).stdout.split()
    out = {}
    for n in nomes:
        if n.endswith('.rs'):
            out[n] = subprocess.run(['git', 'show', '%s:%s' % (commit, n)], cwd=R, capture_output=True, text=True, encoding='utf-8').stdout
    return out


def mapa_de_linhas(velho, novo):
    """{old line: new line} for lines replaced one by one or equal."""
    a, b = velho.split('\n'), novo.split('\n')
    m = {}
    for op, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if op == 'equal' or (op == 'replace' and i2 - i1 == j2 - j1):
            for k in range(i2 - i1):
                m[a[i1 + k]] = b[j1 + k]
    return m


def trocar_como(texto, linhas):
    """Applies the line changes to a text that is a piece of those lines."""
    out = texto
    for velha, nova in linhas.items():
        if velha == nova:
            continue
        # the shared prefix and suffix: the change is the middle
        p = 0
        while p < min(len(velha), len(nova)) and velha[p] == nova[p]:
            p += 1
        s = 0
        while s < min(len(velha), len(nova)) - p and velha[-1 - s] == nova[-1 - s]:
            s += 1
        meio_v, meio_n = velha[p:len(velha) - s], nova[p:len(nova) - s]
        # widen to whole tokens so the middle is findable in the snippet
        ctx_v = velha[max(0, p - 12):len(velha) - s + 12]
        ctx_n = nova[max(0, p - 12):len(nova) - s + 12]
        if ctx_v in out:
            out = out.replace(ctx_v, ctx_n)
        elif meio_v and meio_v in out and out.count(meio_v) == 1:
            out = out.replace(meio_v, meio_n)
    return out


def main():
    commit = sys.argv[1]
    velhas = fontes_em(commit)
    atuais = {n: open(os.path.join(R, n), encoding='utf-8').read() for n in velhas if os.path.exists(os.path.join(R, n))}
    tudo = '\n'.join(atuais.values())
    n_ok, n_falha = 0, 0
    for lf in sorted(os.listdir(PASTA)):
        if not lf.endswith('.py'):
            continue
        cwd = os.getcwd()
        os.chdir(os.path.join(R, 'engenharia-reversa'))
        try:
            ms = runpy.run_path(os.path.join(PASTA, lf)).get('MUTACOES', [])
        except Exception:
            ms = []
        finally:
            os.chdir(cwd)
        txt = open(os.path.join(PASTA, lf), encoding='utf-8').read()
        novo_txt = txt
        for m in ms:
            if tudo.count(m[1]) == 1:
                continue
            arq = [n for n, t in velhas.items() if t.count(m[1]) == 1]
            if len(arq) != 1:
                continue
            v, a = velhas[arq[0]], atuais[arq[0]]
            i = v.index(m[1])
            l0, l1 = v.rfind('\n', 0, i) + 1, v.find('\n', i + len(m[1]))
            linhas_v = v[l0:l1].split('\n')
            mapa = mapa_de_linhas(v, a)
            linhas = {l: mapa[l] for l in linhas_v if l in mapa}
            if len(linhas) != len(linhas_v):
                print('NÃO SEGUI (linha removida)', lf, m[0])
                n_falha += 1
                continue
            ancora, troca = trocar_como(m[1], linhas), trocar_como(m[2], linhas)
            if tudo.count(ancora) != 1:
                print('NÃO SEGUI', lf, m[0], repr(ancora[:100]))
                n_falha += 1
                continue
            ini = max(novo_txt.find("('%s'," % m[0]), novo_txt.find('("%s",' % m[0]))
            pos = ini
            for velho_s, novo_s in ((m[1], ancora), (m[2], troca)):
                if velho_s == novo_s:
                    continue
                for rv, rn in ((repr(velho_s), repr(novo_s)), ('"%s"' % velho_s, '"%s"' % novo_s)):
                    j = novo_txt.find(rv, pos)
                    if j >= 0:
                        novo_txt = novo_txt[:j] + rn + novo_txt[j + len(rv):]
                        pos = j + len(rn)
                        break
            n_ok += 1
            print('segui', lf, m[0])
        if novo_txt != txt and '--gravar' in sys.argv:
            open(os.path.join(PASTA, lf), 'w', encoding='utf-8').write(novo_txt)
    print('%d seguidas, %d não' % (n_ok, n_falha))


if __name__ == '__main__':
    main()
