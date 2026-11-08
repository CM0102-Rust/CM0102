r"""Mutações cujo trecho ficou repetido depois de uma troca de nomes (unificar_memoria.py, unificar_auxiliares.py):
entre as ocorrências, escolhe a que está dentro de uma função do próprio item (o prefixo do nome da mutação, ex.
cqo7) e estende o trecho com as linhas de cima até ele voltar a ser único.

Uso: python desambiguar_mutacoes.py leva279.py [leva280.py ...]
"""
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
from fila import texto_do_motor  # noqa: E402


def main():
    t = texto_do_motor(R)
    for lista in sys.argv[1:]:
        p = os.path.join(R, 'engenharia-reversa', 'mutacoes', lista)
        texto = open(p, encoding='utf-8').read()
        for m in runpy.run_path(p)['MUTACOES']:
            if t.count(m[1]) <= 1:
                continue
            item = m[0].split('_')[0]
            pos = [x.start() for x in re.finditer(re.escape(m[1]), t)]
            dono = [i for i in pos if (re.findall(r'fn (\w+)', t[max(0, i - 20000):i]) or [''])[-1].startswith(item)]
            if len(dono) != 1:
                print(lista, m[0], 'sem dono único:', len(dono))
                continue
            i = k = dono[0]
            while True:
                k = t.rindex('\n', 0, k - 1) + 1
                pre = t[k:i]
                if t.count(pre + m[1]) == 1:
                    break
            ls = texto.split('\n')
            j = next(n for n, l in enumerate(ls) if l.lstrip().startswith("('%s'," % m[0]))
            assert repr(m[1]) in ls[j] and repr(m[2]) in ls[j], (lista, m[0])
            ls[j] = ls[j].replace(repr(m[1]), repr(pre + m[1]), 1).replace(repr(m[2]), repr(pre + m[2]), 1)
            texto = '\n'.join(ls)
            print(lista, m[0], 'ok com', pre.count('\n'), 'linha(s) a mais')
        open(p, 'w', encoding='utf-8').write(texto)


if __name__ == '__main__':
    main()
