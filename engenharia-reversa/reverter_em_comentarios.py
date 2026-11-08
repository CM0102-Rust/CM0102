r"""Desfaz, só nos comentários, a troca dos nomes de uma palavra (auxiliares: amarelo -> give_yellow_card; rotinas:
passar -> pass_ball...): a palavra portuguesa nos comentários em português era palavra, não o nome da função
(correção de 07/11/2026 de aplicar_nomes_auxiliares.py e renomear_rotinas.py, achada pelo leitor de fora, rodada 2).

Nome novo com `_` (pass_ball): volta em qualquer comentário. Nome novo de uma palavra que também é inglês comum
(penalty, corner, shot): volta só nos comentários em português (os cabeçalhos em inglês usam a palavra de propósito).
Vale no motor, nos itens da fila e nas listas de mutação (do mesmo jeito: as âncoras continuam batendo).

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

pares = []
for arq in ('NOMES_AUXILIARES.csv', 'NOMES.csv'):
    p = os.path.join(R, 'docs', arq)
    if os.path.exists(p):
        pares += [(r['en'], r['pt']) for r in csv.DictReader(open(p, encoding='utf-8')) if '_' not in r['pt']]
volta = dict(pares)
compostos = {e: p for e, p in volta.items() if '_' in e}
simples = {e: p for e, p in volta.items() if '_' not in e}
rx_c = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(compostos, key=len, reverse=True))))
rx_s = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(simples, key=len, reverse=True))))
PT = re.compile(r' (?:de|do|da|dos|das|que|com|para|na|no|nos|nas|em|um|uma|ao|à|é|não|pela|pelo|se|ou)\b|ção|ções|ões')


def linha(l):
    cod, sep, com = l.partition('//')
    if not sep:
        return l
    com = rx_c.sub(lambda m: compostos[m.group(1)], com)
    if PT.search(com):
        com = rx_s.sub(lambda m: simples[m.group(1)], com)
    return cod + sep + com


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
