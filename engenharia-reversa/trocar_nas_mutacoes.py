r"""Aplica nas listas de mutação (engenharia-reversa/mutacoes/*.py) a mesma troca de texto feita no motor, para os
trechos continuarem achando o código (usado por nomear_chaves.py e nomear_funcoes.py). A troca é feita por linha do
código Rust: tanto as linhas de verdade quanto as `\n` dentro das strings."""
import os
import re

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def aplicar_por_trecho(trocar_linha):
    """Como aplicar(), mas trecho por trecho: carrega cada lista, aplica trocar_linha a cada linha do texto de cada
    trecho e regrava o literal dentro da tupla da própria mutação (uma linha Python com vários trechos não se mistura).
    Use quando a troca depende do que vem depois de `//` (comentários)."""
    import json
    import runpy
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    n = 0
    for f in sorted(os.listdir(pasta)):
        if not f.endswith('.py'):
            continue
        p = os.path.join(pasta, f)
        try:
            ms = runpy.run_path(p).get('MUTACOES', [])
        except Exception:
            continue
        s = open(p, encoding='utf-8').read()
        mudou = False
        for m in ms:
            ini = max(s.find("('%s'," % m[0]), s.find('("%s",' % m[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                novo = '\n'.join(trocar_linha(l) for l in m[k].split('\n'))
                if novo == m[k]:
                    continue
                for v, nv in ((repr(m[k]), repr(novo)), (json.dumps(m[k], ensure_ascii=False), json.dumps(novo, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + nv + s[j + len(v):]
                        pos = j + len(nv)
                        mudou = True
                        break
        if mudou:
            open(p, 'w', encoding='utf-8').write(s)
            n += 1
    print('listas de mutação trocadas (por trecho):', n)


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
