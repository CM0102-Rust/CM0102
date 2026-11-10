r"""Quais ramos pendentes (docs/PENDENCIAS.md) rodaram numa sessao: le o jogo\ddraw.log.

Uso: python pendencias.py [ddraw.log]
"""
import os
import re
import sys

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
log = sys.argv[1] if len(sys.argv) > 1 else os.path.join(RAIZ, 'jogo', 'ddraw.log')
pendentes = re.findall(r'^\| `([^`]+)` \|', open(os.path.join(RAIZ, 'docs', 'PENDENCIAS.md'), encoding='utf-8').read(), re.M)
texto = open(log, encoding='utf-8', errors='replace').read()
vistos = {}
for nome, n in re.findall(r'^(?:ramo|branch) (.+): (\d+)$', texto, re.M):
    vistos[nome] = max(vistos.get(nome, 0), int(n))
for p in pendentes:
    print('%-28s %s' % (p, 'RODOU (%d+)' % vistos[p] if p in vistos else 'ainda pendente'))
print('divergencias no registro:', len(re.findall('DIVERGEN(?:CIA|CE)', texto)))
