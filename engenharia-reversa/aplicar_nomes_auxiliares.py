r"""Aplica os nomes em inglês propostos para os auxiliares do motor (etapa 4 da reorganização, rodada 3, 07/11/2026).

Entrada: arquivos TSV `<nome velho>\t<nome novo>\t<descrição em inglês>` (propostos por leitura do código; ninguém
edita o motor à mão). Cada nome novo é conferido (snake_case ASCII, sem palavra reservada); se já existe no código ou
se repete em outro arquivo, leva o nome do arquivo na frente; se ainda repetir, o nome velho no fim. A descrição vira a
primeira linha do comentário (o comentário antigo fica como `[pt]`). A troca vale no motor, nos itens da fila e nas
listas de mutação; o mapa vai para docs/NOMES_AUXILIARES.csv.

Uso: python aplicar_nomes_auxiliares.py arquivo1.tsv [arquivo2.tsv ...] [--gravar]
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import renomear_rotinas  # noqa: E402
SAIDA = os.path.join(R, 'docs', 'NOMES_AUXILIARES.csv')


def quebrar(texto, largura=116):
    linhas, atual = [], ''
    for p in texto.split():
        if atual and len(atual) + 1 + len(p) > largura:
            linhas.append(atual)
            atual = p
        else:
            atual = (atual + ' ' + p).strip()
    return linhas + ([atual] if atual else [])


def main():
    tsvs = [a for a in sys.argv[1:] if not a.startswith('--')]
    propostas = {}
    for t in tsvs:
        for l in open(t, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            if len(p) >= 3 and p[0] and p[1]:
                propostas[p[0].strip()] = (p[1].strip(), p[2].strip())
    # onde cada auxiliar está definido
    arquivo_de, textos = {}, {}
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                p = os.path.join(d, f)
                textos[p] = open(p, encoding='utf-8').read()
                for n, _, _ in mover_rotinas.blocos(textos[p]):
                    arquivo_de.setdefault(n, p)
    codigo = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', '\n'.join(open(p, encoding='utf-8').read()
                                                                  for p in renomear_rotinas.fontes()))
    idents = set(re.findall(r'\b[A-Za-z_]\w*\b', codigo))
    troca, desc, recusas, usados = {}, {}, [], set()
    conta = {}
    for velho, (novo, _) in propostas.items():
        conta[novo] = conta.get(novo, 0) + 1
    for velho, (novo, d) in sorted(propostas.items()):
        if velho not in arquivo_de:
            recusas.append((velho, 'não existe'))
            continue
        if not re.fullmatch(r'[a-z][a-z0-9_]*', novo) or novo in renomear_rotinas.RESERVADAS:
            recusas.append((velho, 'nome inválido: ' + novo))
            continue
        stem = os.path.splitext(os.path.basename(arquivo_de[velho]))[0]
        if conta[novo] > 1 or novo in idents or novo in usados:
            novo = '%s_%s' % (stem, novo)
        if novo in idents or novo in usados:
            novo = '%s_%s' % (novo, velho)
        usados.add(novo)
        troca[velho] = novo
        desc[velho] = d
    print('%d nomes aplicáveis, %d recusados' % (len(troca), len(recusas)))
    for x in recusas[:15]:
        print('  recusado:', x)
    if '--gravar' not in sys.argv:
        return
    # 1) a descrição em inglês em cima de cada auxiliar
    for p, t in textos.items():
        partes, pos = [], 0
        for n, a, b in mover_rotinas.blocos(t):
            if n not in troca or not desc.get(n):
                continue
            bloco = t[a:b]
            corpo_ini = re.search(r'^(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn ', bloco, re.M).start()
            topo = bloco[:corpo_ini].split('\n')[:-1]
            docs = [l for l in topo if l.lstrip().startswith('///')]
            attrs = [l for l in topo if l.lstrip().startswith('#[')]
            if any('[pt]' in l for l in docs):
                continue
            cab = ['/// ' + l for l in quebrar(desc[n])]
            if docs:
                cab.append('///')
                docs[0] = docs[0].replace('/// ', '/// [pt] ', 1) if docs[0].startswith('/// ') else docs[0]
                cab += docs
            partes.append(t[pos:a])
            partes.append('\n'.join(cab + attrs) + '\n' + bloco[corpo_ini:])
            pos = b
        partes.append(t[pos:])
        textos[p] = ''.join(partes)
    # 2) os nomes, em todos os fontes, nos itens da fila e nas listas de mutação
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
    sub = lambda t: rx.sub(lambda m: troca[m.group(1)], t)
    for p in renomear_rotinas.fontes():
        t = textos.get(p) or open(p, encoding='utf-8').read()
        novo = sub(t)
        if novo != open(p, encoding='utf-8').read():
            open(p, 'w', encoding='utf-8').write(novo)
    fila = os.path.join(R, 'levas', 'fila')
    for item in os.listdir(fila):
        for f in ('codigo.rs', 'mutacoes.py'):
            p = os.path.join(fila, item, f)
            if os.path.exists(p):
                t = open(p, encoding='utf-8').read()
                if sub(t) != t:
                    open(p, 'w', encoding='utf-8').write(sub(t))
    import trocar_nas_mutacoes
    trocar_nas_mutacoes.aplicar(sub)
    antigos = list(csv.DictReader(open(SAIDA, encoding='utf-8'))) if os.path.exists(SAIDA) else []
    with open(SAIDA, 'w', encoding='utf-8', newline='') as f:
        w = csv.DictWriter(f, ['pt', 'en', 'arquivo', 'descricao'])
        w.writeheader()
        w.writerows(antigos)
        for v, n in sorted(troca.items()):
            w.writerow({'pt': v, 'en': n, 'arquivo': os.path.relpath(arquivo_de[v], R).replace(os.sep, '/'), 'descricao': desc[v]})
    print('gravado')


if __name__ == '__main__':
    main()
