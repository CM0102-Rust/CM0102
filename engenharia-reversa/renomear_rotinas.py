r"""Dá nome em inglês às rotinas portadas (etapa 4 da reorganização, rodada 3, 07/11/2026).

O nome novo é o do método original no mapa de maxevans60 (`TRANSFER_MANAGER::add_transfer_offer` ->
`add_transfer_offer`; construtor `construct_<classe>`, destrutor `destroy_<classe>`). Se o nome se repete entre
arquivos (as ligas têm os mesmos métodos virtuais), ou já existe no código, ou é palavra reservada, leva o arquivo na
frente (`eng_prm_set_up_rules`); se ainda assim repetir, o endereço no fim. O comparador acompanha
(`compare_<nome novo>`) e a constante de índice também (IDX_<NOME NOVO>, do build.rs). Os rótulos de texto ("motor_...")
ficam como estão (as ferramentas da prova do save os usam).

O mapa velho -> novo vai para docs/NOMES.csv (as ferramentas e a documentação ainda chamam as rotinas pelo nome em
português) e é aplicado ao motor, aos outros fontes do rust/src, aos itens da fila e às listas de mutação.
Rodar de novo renomeia só as rotinas que entraram depois com nome em português.

Uso: python renomear_rotinas.py [--gravar]
"""
import collections
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import cabecalhos  # noqa: E402
import mover_rotinas  # noqa: E402
RESERVADAS = set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub '
                 'ref return self static struct super trait true type unsafe use where while async await dyn abstract '
                 'become box do final macro override priv typeof unsized virtual yield try union'.split())
NOMES = os.path.join(R, 'docs', 'NOMES.csv')


def snake(x):
    x = re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', x)
    return re.sub(r'[^a-zA-Z0-9]+', '_', x).strip('_').lower()


def fontes():
    for d, _, fs in os.walk(SRC):
        if os.sep + 'target' in d:
            continue
        for f in fs:
            if f.endswith('.rs'):
                yield os.path.join(d, f)


def main():
    orig = cabecalhos.originais()
    s = open(os.path.join(SRC, 'motor.rs'), encoding='utf-8').read()
    import tabela_motor
    nomes = tabela_motor.nomes_rust(s)
    ja = {}
    if os.path.exists(NOMES):
        ja = {r['pt']: r['en'] for r in csv.DictReader(open(NOMES, encoding='utf-8'))}
    feitos = set(ja.values())
    # o módulo de cada rotina (para o prefixo do arquivo)
    modulo = {}
    for p in fontes():
        if os.sep + 'motor' + os.sep in p:
            t = open(p, encoding='utf-8').read()
            for n, _, _ in mover_rotinas.blocos(t):
                modulo[n] = os.path.splitext(os.path.basename(p))[0]
    # os identificadores do código (sem comentários nem textos entre aspas: os cabeçalhos em inglês têm palavras como
    # "limit" que não são nomes)
    codigo = '\n'.join(open(p, encoding='utf-8').read() for p in fontes())
    codigo = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', codigo)
    idents = set(re.findall(r'\b[A-Za-z_]\w*\b', codigo))
    end_de = dict(zip(nomes, re.findall(r'Prologue \{ address: (0x[0-9a-f]+)', s[:s.index('const N')])))
    candidatos = {}
    for n in nomes:
        if n in feitos or n in ja or n not in modulo:  # só as definidas no motor/ (as do cm0102-base ficam)
            continue
        o = orig.get(n, '')
        if not o or 'sem nome' in o:
            continue
        cls, _, met = o.rpartition('::')
        if met.startswith('~'):
            base = 'destroy_' + snake(cls)
        elif cls and met == cls.split('::')[-1]:
            base = 'construct_' + snake(met)
        else:
            base = snake(met)
        candidatos[n] = base
    conta = collections.Counter(candidatos.values())
    troca, usados = {}, set(feitos)
    for n, base in candidatos.items():
        novo = base
        if conta[base] > 1 or novo in RESERVADAS or novo in idents or novo in usados:
            novo = '%s_%s' % (modulo.get(n, 'motor'), base)
        if novo in usados or novo in idents:
            novo = '%s_%s' % (novo, end_de.get(n, '0x0')[2:])  # último recurso: o endereço no fim
        usados.add(novo)
        troca[n] = novo
    print('%d rotinas renomeadas (%d com o arquivo na frente)' % (len(troca), sum(1 for n, v in troca.items() if v != candidatos[n])))
    for n in list(troca)[:10]:
        print('  %s -> %s' % (n, troca[n]))
    if '--gravar' not in sys.argv:
        return
    mapa = dict(troca)
    mapa.update({'compare_' + a: 'compare_' + b for a, b in troca.items()})
    mapa.update({'IDX_' + a.upper(): 'IDX_' + b.upper() for a, b in troca.items()})
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(mapa, key=len, reverse=True))))

    def trocar(t):
        # os rótulos de texto ("motor_<nome>") não mudam sozinhos: o nome vem grudado em "motor_" (sem fronteira \b)
        return rx.sub(lambda m: mapa[m.group(1)], t)

    for p in fontes():
        t = open(p, encoding='utf-8').read()
        novo = trocar(t)
        if novo != t:
            open(p, 'w', encoding='utf-8').write(novo)
    fila = os.path.join(R, 'levas', 'fila')
    for item in os.listdir(fila):
        for f in ('codigo.rs', 'mutacoes.py'):
            p = os.path.join(fila, item, f)
            if os.path.exists(p):
                t = open(p, encoding='utf-8').read()
                novo = trocar(t) if f == 'codigo.rs' else rx.sub(lambda m: mapa[m.group(1)], t)
                if novo != t:
                    open(p, 'w', encoding='utf-8').write(novo)
    import trocar_nas_mutacoes
    trocar_nas_mutacoes.aplicar(lambda l: rx.sub(lambda m: mapa[m.group(1)], l))
    end = dict(zip(nomes, re.findall(r'Prologue \{ address: (0x[0-9a-f]+)', s[:s.index('const N')])))
    linhas = [{'pt': a, 'en': b, 'endereco': end.get(a, ''), 'original': orig.get(a, '')} for a, b in ja.items()]
    linhas += [{'pt': a, 'en': b, 'endereco': end.get(a, ''), 'original': orig.get(a, '')} for a, b in troca.items()]
    with open(NOMES, 'w', encoding='utf-8', newline='') as f:
        w = csv.DictWriter(f, ['pt', 'en', 'endereco', 'original'])
        w.writeheader()
        w.writerows(linhas)
    print('gravado; docs/NOMES.csv com %d nomes' % len(linhas))


if __name__ == '__main__':
    main()
