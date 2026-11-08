r"""Desfaz, só nos comentários, a troca dos auxiliares de uma palavra (amarelo -> give_yellow_card...): a palavra
portuguesa nos comentários em português era palavra, não o nome da função (correção de 07/11/2026 da
aplicar_nomes_auxiliares.py). Vale no motor, nos itens da fila e nas listas de mutação (do mesmo jeito: as âncoras
continuam batendo).

Uso: python reverter_em_comentarios.py
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import renomear_rotinas  # noqa: E402
import trocar_nas_mutacoes  # noqa: E402

pares = [(r['en'], r['pt']) for r in csv.DictReader(open(os.path.join(R, 'docs', 'NOMES_AUXILIARES.csv'), encoding='utf-8'))
         if '_' not in r['pt']]
volta = dict(pares)
rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(volta, key=len, reverse=True))))


def linha(l):
    cod, sep, com = l.partition('//')
    return cod + sep + rx.sub(lambda m: volta[m.group(1)], com) if sep else l


n = 0
alvos = list(renomear_rotinas.fontes())
fila = os.path.join(R, 'levas', 'fila')
alvos += [os.path.join(fila, i, 'codigo.rs') for i in os.listdir(fila) if os.path.exists(os.path.join(fila, i, 'codigo.rs'))]
for p in alvos:
    t = open(p, encoding='utf-8').read()
    novo = '\n'.join(linha(l) for l in t.split('\n'))
    if novo != t:
        open(p, 'w', encoding='utf-8').write(novo)
        n += 1
trocar_nas_mutacoes.aplicar(linha)
print('%d fontes corrigidos' % n)
