r"""Aplica nas listas de mutação (engenharia-reversa/mutacoes/*.py) a mesma troca de texto feita no motor, para os
trechos continuarem achando o código (usado por nomear_chaves.py e nomear_funcoes.py). A troca é feita por linha do
código Rust: tanto as linhas de verdade quanto as `\n` dentro das strings."""
import os
import re

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def aplicar(trocar_linha):
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    n = 0
    for f in os.listdir(pasta):
        if not f.endswith('.py'):
            continue
        p = os.path.join(pasta, f)
        t = open(p, encoding='utf-8').read()
        partes = re.split(r'(\n|\\n)', t)
        novo = ''.join(x if x in ('\n', '\\n') else trocar_linha(x) for x in partes)
        if novo != t:
            open(p, 'w', encoding='utf-8').write(novo)
            n += 1
    print('listas de mutação trocadas:', n)
