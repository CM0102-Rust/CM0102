"""Removes the stray spaces cargo fix leaves where it dropped parentheses (`x as u8 ;`, `f(x as i32 )`), only on
lines changed since COMMIT and outside string literals and comments (readability batch 33; run by padronizar.sh).
Usage: python espacos_do_cargo_fix.py COMMIT
[pt] Tira os espaços que o cargo fix deixa onde tirou parênteses, só nas linhas mudadas desde COMMIT."""
import re
import subprocess
import sys
import os
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
base = sys.argv[1]
arqs = subprocess.run(['git', 'diff', '--name-only', base, '--', 'rust/src'], cwd=R, capture_output=True, text=True).stdout.split()
n = 0
for a in arqs:
    antes = set(subprocess.run(['git', 'show', '%s:%s' % (base, a)], cwd=R, capture_output=True, text=True, encoding='utf-8').stdout.split('\n'))
    p = R + '/' + a
    linhas = open(p, encoding='utf-8', newline='').read().split('\n')
    for i, l in enumerate(linhas):
        if l.rstrip('\r') in antes:
            continue
        cod, sep, com = l.partition('//')
        if '"' in cod:
            continue
        novo = re.sub(r'(?<=\S) +(?=[;)\]])', '', cod)
        novo = re.sub(r'(?<=[(\[]) +(?=\S)', '', novo)
        if novo != cod:
            linhas[i] = novo + (' ' if sep and not novo.endswith(' ') else '') + sep + com if sep else novo
            n += 1
    open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))
print(n, 'linhas')
