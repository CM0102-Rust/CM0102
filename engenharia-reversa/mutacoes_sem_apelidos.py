r"""Mutations whose replacement says `let _ = alias;` for a local alias that no longer exists (the round 25 wrappers
turned `let alias = transmute(FN_X)` into `exe::x(..)`): drop the missing names from the `let _ = (..)`, so the
mutation still compiles. Prints what it changes; --gravar writes.
[pt] Mutações cujo texto de troca cita um apelido local que saiu (rodada 25): tira do `let _ = (..)` os nomes que não
existem mais na função, para a mutação continuar compilando.
"""
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402

LET_ = re.compile(r'let _ = (\(?)([\w, ]+?)(\)?);[ \t]*')


def existe(x, texto):
    return re.search(r'(?<![\w.])%s\b' % re.escape(x), texto) is not None


def main():
    fontes = {}
    for dp, _, fs in os.walk(os.path.join(R, 'rust', 'src')):
        for f in fs:
            if f.endswith('.rs'):
                fontes[os.path.join(dp, f)] = open(os.path.join(dp, f), encoding='utf-8').read()
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    n = 0
    for lf in sorted(os.listdir(pasta)):
        if not lf.endswith('.py'):
            continue
        try:
            ms = runpy.run_path(os.path.join(pasta, lf)).get('MUTACOES', [])
        except Exception:
            continue
        txt = open(os.path.join(pasta, lf), encoding='utf-8').read()
        novo_txt = txt
        for m in ms:
            corpo = None
            for t in fontes.values():
                i = t.find(m[1])
                if i >= 0:
                    corpo = next((t[a:b] for _, a, b in mover_rotinas.blocos(t) if a <= i < b), None)
                    break
            if corpo is None:
                continue
            resto = re.sub(r'//[^\n]*', '', corpo.replace(m[1], '') + '\n' + m[1])

            def arruma(mm):
                ids = [x.strip() for x in mm.group(2).split(',') if x.strip()]
                vivos = [x for x in ids if existe(x, resto)]
                if vivos == ids:
                    return mm.group(0)
                if not vivos:
                    return ''
                return 'let _ = %s; ' % (vivos[0] if len(vivos) == 1 else '(%s)' % ', '.join(vivos))
            nova = LET_.sub(arruma, m[2])
            if nova != m[2] and not nova.strip():
                nova = m[2][:len(m[2]) - len(m[2].lstrip())] + '// mutacao'
            if nova != m[2]:
                ini = max(novo_txt.find("('%s'," % m[0]), novo_txt.find('("%s",' % m[0]))
                for v, w in ((repr(m[2]), repr(nova)), ('"%s"' % m[2], '"%s"' % nova)):
                    j = novo_txt.find(v, ini) if ini >= 0 else -1
                    if j >= 0:
                        novo_txt = novo_txt[:j] + w + novo_txt[j + len(v):]
                        n += 1
                        print(lf, m[0], '|', nova.strip()[:110])
                        break
                else:
                    print('NAO ACHEI', lf, m[0])
        if novo_txt != txt and '--gravar' in sys.argv:
            open(os.path.join(pasta, lf), 'w', encoding='utf-8').write(novo_txt)
    print(n, 'mutações')


if __name__ == '__main__':
    main()
