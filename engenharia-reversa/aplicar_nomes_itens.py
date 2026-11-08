r"""Aplica os nomes em inglês propostos para os itens de topo do motor (constantes, estáticas, structs, enums; etapa 4
da reorganização, 08/11/2026). Entrada: TSV `<nome velho>\t<nome novo>`. Confere o estilo (constante/estática em
MAIÚSCULAS_COM_SUBLINHADO, struct/enum/type em CamelCase); se o nome já existe no código ou se repete, leva o nome do
arquivo na frente. A troca vale no motor, nos itens da fila e nas listas de mutação; o mapa vai para docs/NOMES_ITENS.csv.

Uso: python aplicar_nomes_itens.py arquivo1.tsv [...] [--gravar]
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import renomear_rotinas  # noqa: E402
import trocar_nas_mutacoes  # noqa: E402
FORA = ('keys.rs', 'globals.rs', 'exe_fns.rs', 'fields.rs', 'mem.rs')


def camel(s):
    return ''.join(p[:1].upper() + p[1:] for p in re.split(r'[_\W]+', s) if p)


def main():
    propostas = {}
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        for l in open(t, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            if len(p) >= 2 and p[0].strip() and p[1].strip() and p[0].strip() != p[1].strip():
                propostas[p[0].strip()] = p[1].strip()
    tipo, arquivo = {}, {}
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs') and f not in FORA:
                p = os.path.join(d, f)
                for k, n in re.findall(r'^pub\(crate\) (const(?! fn)|static(?: mut)?|struct|enum|type) ([A-Za-z_]\w*)', open(p, encoding='utf-8').read(), re.M):
                    tipo.setdefault(n, k)
                    arquivo.setdefault(n, p)
    codigo = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', '\n'.join(open(p, encoding='utf-8').read() for p in renomear_rotinas.fontes()))
    idents = set(re.findall(r'\b[A-Za-z_]\w*\b', codigo))
    conta = {}
    for n in propostas.values():
        conta[n] = conta.get(n, 0) + 1
    troca, recusas, usados = {}, [], set()
    for velho, novo in sorted(propostas.items()):
        k = tipo.get(velho)
        if not k:
            recusas.append((velho, 'não existe'))
            continue
        ok = re.fullmatch(r'[A-Z][A-Z0-9_]*', novo) if k.split()[0] in ('const', 'static') else re.fullmatch(r'[A-Z][A-Za-z0-9]*', novo)
        if not ok:
            recusas.append((velho, 'estilo: ' + novo))
            continue
        stem = os.path.splitext(os.path.basename(arquivo[velho]))[0]
        if conta[novo] > 1 or novo in idents or novo in usados:
            novo = (stem.upper() + '_' + novo) if k.split()[0] in ('const', 'static') else camel(stem) + novo
        if novo in idents or novo in usados:
            recusas.append((velho, 'repetido: ' + novo))
            continue
        usados.add(novo)
        troca[velho] = novo
    print('%d nomes aplicáveis, %d recusados' % (len(troca), len(recusas)))
    for x in recusas[:12]:
        print('  recusado:', x)
    if '--gravar' not in sys.argv:
        return
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
    sub = lambda t: rx.sub(lambda m: troca[m.group(1)], t)
    for p in renomear_rotinas.fontes():
        t = open(p, encoding='utf-8').read()
        if sub(t) != t:
            open(p, 'w', encoding='utf-8').write(sub(t))
    fila = os.path.join(R, 'levas', 'fila')
    for item in os.listdir(fila):
        for f in ('codigo.rs', 'mutacoes.py'):
            p = os.path.join(fila, item, f)
            if os.path.exists(p):
                t = open(p, encoding='utf-8').read()
                if sub(t) != t:
                    open(p, 'w', encoding='utf-8').write(sub(t))
    trocar_nas_mutacoes.aplicar(sub)
    with open(os.path.join(R, 'docs', 'NOMES_ITENS.csv'), 'w', encoding='utf-8', newline='') as f:
        w = csv.writer(f)
        w.writerow(['pt', 'en', 'tipo', 'arquivo'])
        for v, n in sorted(troca.items()):
            w.writerow([v, n, tipo[v], os.path.relpath(arquivo[v], R).replace(os.sep, '/')])
    print('gravado')


if __name__ == '__main__':
    main()
