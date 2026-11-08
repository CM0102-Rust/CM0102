r"""Restaura, nos comentários, as palavras ambíguas (passar/pass_ball, jogada/play, penalti/penalty...) como estavam
no texto antes das trocas de nome (correção de 07/11/2026; nenhuma heurística resolve: "play-off" é inglês dentro do
português, "no promotion" é inglês). Para cada linha de comentário com uma dessas palavras, acha a mesma linha na
árvore de antes (normalizando as duas: todos os nomes trocados para o inglês e as palavras ambíguas viram marcador) e
copia de lá a palavra de cada posição. Vale nos fontes, nos itens da fila e nas listas de mutação.

Uso: python restaurar_palavras.py <commit de antes das trocas>
"""
import csv
import os
import re
import subprocess
import sys
import tempfile

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import renomear_rotinas  # noqa: E402
import trocar_nas_mutacoes  # noqa: E402

pares = []
for arq in ('NOMES_AUXILIARES.csv', 'NOMES.csv'):
    pares += [(r['pt'], r['en']) for r in csv.DictReader(open(os.path.join(R, 'docs', arq), encoding='utf-8'))]
ida = dict(pares)
rx_ida = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(ida, key=len, reverse=True))))
ambiguas = set()
for p, e in pares:
    if '_' not in p:
        ambiguas.add(p)
        ambiguas.add(e)
rx_amb = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(ambiguas, key=len, reverse=True))))


def chave(com):
    # o "[pt] " que os cabeçalhos acrescentaram na primeira linha do comentário antigo não conta
    return rx_amb.sub('\x00', rx_ida.sub(lambda m: ida[m.group(1)], com.replace('[pt] ', '').strip()))


def coletar(textos):
    d = {}
    for t in textos:
        for l in t.split('\n'):
            _, sep, com = l.partition('//')
            if sep and rx_amb.search(com) or (sep and rx_ida.search(com)):
                k = chave(com)
                if '\x00' in k:
                    d.setdefault(k, set()).add(com)
    return d


def main():
    commit = sys.argv[1]
    tmp = tempfile.mkdtemp()
    subprocess.run('git archive %s rust/src levas/fila engenharia-reversa/mutacoes | tar -x -C "%s"' % (commit, tmp.replace('\\', '/')),
                   shell=True, cwd=R, check=True)
    antigos = []
    for d, _, fs in os.walk(tmp):
        for f in fs:
            if f.endswith(('.rs', '.py')):
                antigos.append(open(os.path.join(d, f), encoding='utf-8', errors='replace').read())
    # as listas de mutação guardam as linhas com \n dentro das strings
    antigos = [t.replace('\\n', '\n') for t in antigos]
    orig = coletar(antigos)

    def linha(l):
        cod, sep, com = l.partition('//')
        if not sep or not rx_amb.search(com):
            return l
        k = chave(com)
        cands = orig.get(k)
        if not cands or len(cands) != 1:
            return l
        o = next(iter(cands))
        palavras = [m.group(1) for m in rx_amb.finditer(rx_ida.sub(lambda m: ida[m.group(1)] if m.group(1) not in ambiguas else m.group(1), o))]
        if len(palavras) != len(rx_amb.findall(com)):
            return l
        it = iter(palavras)
        novo = rx_amb.sub(lambda m: next(it), com)
        return cod + sep + novo

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
    print('%d fontes restaurados' % n)


if __name__ == '__main__':
    main()
