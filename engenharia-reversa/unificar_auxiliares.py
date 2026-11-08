r"""Junta os auxiliares repetidos do motor (cps5_sortear = cps6_sortear = sortear...) numa função só (etapa 4 da
reorganização, rodada 2, 07/11/2026).

Só junta funções de topo com o mesmo texto (assinatura e corpo, com o próprio nome trocado): a cópia fica igual por
construção. Fica o nome sem prefixo de item, se houver; senão o mais curto. As rotinas portadas (as da tabela
PROLOGOS e os comparadores) não entram. As chamadas e as listas de mutação são renomeadas.

Uso: python unificar_auxiliares.py [--gravar]
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
PREFIXO = re.compile(r'^c[a-z]{2}\d+[a-z]?_|^c[a-z]{2}_|^d[a-z]{1,2}\d*[a-z]?_')


def main():
    s = open(os.path.join(SRC, 'motor.rs'), encoding='utf-8').read()
    portadas = set(re.findall(r'(\w+) as \*const', s[s.index('let rust: [usize; N] = ['):]))
    arquivos = {}
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                arquivos[os.path.join(d, f)] = open(os.path.join(d, f), encoding='utf-8').read()
    grupos = collections.defaultdict(list)
    for p, t in arquivos.items():
        for nome, a, b in mover_rotinas.blocos(t):
            if nome in portadas or nome.startswith('comparar_'):
                continue
            corpo = t[a:b]
            # o texto sem os comentários de cima e com o próprio nome trocado
            sem_doc = re.sub(r'^\s*(///|#\[)[^\n]*\n', '', corpo, flags=re.M)
            chave = re.sub(r'\b%s\b' % re.escape(nome), '@', sem_doc)
            grupos[chave].append((nome, p))
    troca = {}
    for chave, membros in grupos.items():
        nomes = sorted({n for n, _ in membros})
        if len(nomes) < 2:
            continue
        limpos = [n for n in nomes if not PREFIXO.match(n)]
        fica = min(limpos or nomes, key=lambda n: (len(n), n))
        for n in nomes:
            if n != fica:
                troca[n] = fica
    print('%d auxiliares repetidos saem (ficam %d)' % (len(troca), len(set(troca.values()))))
    for n, f in list(troca.items())[:12]:
        print('  %s -> %s' % (n, f))
    if '--gravar' not in sys.argv:
        return
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
    for p, t in arquivos.items():
        partes, pos = [], 0
        for nome, a, b in mover_rotinas.blocos(t):
            if nome in troca:  # a definição repetida sai
                partes.append(t[pos:a])
                pos = b
        partes.append(t[pos:])
        novo = rx.sub(lambda m: troca[m.group(1)], ''.join(partes))
        if novo != t:
            open(p, 'w', encoding='utf-8').write(novo)
    m = os.path.join(SRC, 'motor.rs')
    t = open(m, encoding='utf-8').read()
    novo = rx.sub(lambda x: troca[x.group(1)], t)
    if novo != t:
        open(m, 'w', encoding='utf-8').write(novo)
    import trocar_nas_mutacoes
    trocar_nas_mutacoes.aplicar(lambda l: rx.sub(lambda x: troca[x.group(1)], l))
    print('gravado')


if __name__ == '__main__':
    main()
