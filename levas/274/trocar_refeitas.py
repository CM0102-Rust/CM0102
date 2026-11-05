"""Troca na cadeia.txt as linhas 'SEM RESULTADO' (trava do laboratório ocupada, 05/11) pelas mutações refeitas na bancada 3."""
import os
import re

AQUI = os.path.dirname(os.path.abspath(__file__))
refeitas = {}
for arq in ('refazer_mutacoes.txt', 'refazer_mutacoes2.txt'):
    for l in open(os.path.join(AQUI, arq), encoding='utf-8'):
        if l.startswith('mutacao '):
            refeitas.setdefault(l.split()[1], []).append(l.rstrip('\n'))
saida = []
for l in open(os.path.join(AQUI, 'cadeia.txt'), encoding='utf-8'):
    m = re.match(r'mutacao (\S+) .*SEM RESULTADO', l)
    if m:
        saida += [r + ' (refeita: a trava do laboratorio estava ocupada)' for r in refeitas.pop(m.group(1))]
    else:
        saida.append(l.rstrip('\n'))
assert not refeitas, ('sobrou', refeitas)
assert not any('SEM RESULTADO' in l for l in saida)
open(os.path.join(AQUI, 'cadeia.txt'), 'w', encoding='utf-8').write('\n'.join(saida) + '\n')
print('trocadas')
