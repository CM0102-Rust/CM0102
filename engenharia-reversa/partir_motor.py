r"""Parte o rust/src/motor.rs em arquivos por arquivo de origem do jogo (etapa 3 da reorganização, 07/11/2026).

O motor.rs tinha uma seção por leva ("// ---------- Nª leva: ... ----------"), na ordem em que as levas entraram. Cada
seção vai para rust/src/motor/<pasta do .cpp>/<arquivo>.rs pelo arquivo de origem (docs/mapa_funcoes.csv) da maioria
das rotinas que ela porta; a ordem entre as seções do mesmo arquivo se mantém. As macros (macro_rules!) vão para
motor/macros.rs, incluído primeiro (o escopo delas é textual). No motor.rs ficam o cabeçalho, a seção da instalação (as
tabelas e o laço que as ferramentas leem) e a linha-âncora do integrar.py; o resto entra por include! (o mesmo módulo:
nada muda no código gerado). Os módulos de verdade vêm na etapa 4.

Uso: python partir_motor.py          (mostra o plano)
     python partir_motor.py --gravar (grava)
"""
import collections
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
ANCORA = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
SECAO = re.compile(r'^// ---------- .* ----------$', re.M)


def mapa():
    """{endereço: caminho do .cpp}: o docs/mapa_funcoes.csv e, onde ele não tem arquivo, o mapa de maxevans60
    (só o nome do arquivo: a pasta vem do mesmo nome no nosso mapa)."""
    linhas = [r for r in csv.reader(open(os.path.join(R, 'docs', 'mapa_funcoes.csv'), encoding='utf-8')) if r and r[0].startswith('0x')]
    m = {int(r[0], 16): r[2] for r in linhas if r[2]}
    pasta = {os.path.basename(c).lower(): c for c in m.values()}
    tsv = os.path.join(R, 'referencias', 'cm0102-function-map', 'map', 'functions.tsv')
    if os.path.exists(tsv):
        for r in csv.reader(open(tsv, encoding='utf-8'), delimiter='\t'):
            if r and r[0].startswith('0x') and len(r) > 2 and r[2].endswith('.cpp'):
                m.setdefault(int(r[0], 16), pasta.get(r[2].lower(), r[2]))
    return m


def destino(cpp):
    p = cpp.replace('\\', '/').lower()
    for pre in ('cm3/code/', 'cm3/', 'si/code/', 'si/'):
        if p.startswith(pre):
            p = p[len(pre):]
            break
    if not p:
        return 'unmapped.rs'  # sem arquivo no mapa
    return re.sub(r'\.(cpp|c|h)$', '', p).replace('.', '_').replace('-', '_') + '.rs'


def main():
    s = open(os.path.join(SRC, 'motor.rs'), encoding='utf-8').read()
    nomes = re.findall(r'(\w+) as \*const', s[s.index('let rust: [usize; N] = ['):])
    pro = re.findall(r'Prologo \{ endereco: (0x[0-9a-f]+)', s[:s.index('const N')])
    end_de = dict(zip(nomes[:len(pro)], (int(a, 16) for a in pro)))
    arquivo = mapa()
    cab_fim = SECAO.search(s).start()
    cabeca, corpo = s[:cab_fim], s[cab_fim:]
    inicios = [m.start() for m in SECAO.finditer(corpo)] + [len(corpo)]
    secoes = [corpo[a:b] for a, b in zip(inicios, inicios[1:])]
    fica, plano = [], collections.OrderedDict()
    macros = []
    for sec in secoes:
        titulo = sec.split('\n', 1)[0]
        # as macros saem da seção (escopo textual: vão todas para o começo)
        for m in list(re.finditer(r'^macro_rules! \w+ \{\n.*?^\}\n', sec, re.M | re.S)):
            macros.append(m.group(0))
        sec = re.sub(r'^macro_rules! \w+ \{\n.*?^\}\n', '', sec, flags=re.M | re.S)
        if 'pub unsafe fn instalar(' in sec or ANCORA in sec:
            fica.append(sec)
            continue
        votos = collections.Counter()
        for f in re.findall(r'^(?:pub )?(?:unsafe )?(?:extern "\w+" )?fn (\w+)', sec, re.M):
            if f in end_de and end_de[f] in arquivo:
                votos[destino(arquivo[end_de[f]])] += 1
        alvo = votos.most_common(1)[0][0] if votos else 'shared.rs'
        plano.setdefault(alvo, []).append((titulo, sec))
    if '--gravar' not in sys.argv:
        for alvo, lst in sorted(plano.items(), key=lambda x: -sum(len(t) for _, t in x[1])):
            print('%-50s %3d seções %7d linhas' % (alvo, len(lst), sum(t.count('\n') for _, t in lst)))
        print('ficam no motor.rs:', [t.split('\n', 1)[0][:70] for t in fica])
        print('macros:', len(macros), '; arquivos:', len(plano))
        return
    base = os.path.join(SRC, 'motor')
    os.makedirs(base, exist_ok=True)
    # de novo (o motor.rs já partido, com código novo de levas): acrescenta aos arquivos que existem
    pm = os.path.join(base, 'macros.rs')
    antes = open(pm, encoding='utf-8').read() if os.path.exists(pm) else \
        '// The match-engine macros (textual scope: included first). [pt] As macros do motor (escopo textual: incluídas primeiro).\n\n'
    open(pm, 'w', encoding='utf-8').write(antes + ''.join('\n' + m for m in macros if m not in antes))
    for alvo, lst in sorted(plano.items()):
        p = os.path.join(base, alvo)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        # include! é relativo ao arquivo que o contém: os gerados (chaves_fixas.rs...) ficam em src/
        sobe = '../' * (alvo.count('/') + 1)
        texto = re.sub(r'include!\("(\w+\.rs)"\)', lambda m: 'include!("%s%s")' % (sobe, m.group(1)), ''.join(t for _, t in lst))
        with open(p, 'a', encoding='utf-8') as f:
            f.write(texto)
    todos = sorted(os.path.relpath(os.path.join(d, f), base).replace('\\', '/') for d, _, fs in os.walk(base) for f in fs if f.endswith('.rs'))
    inc = ['include!("motor/macros.rs");'] + ['include!("motor/%s");' % a for a in todos if a != 'macros.rs']
    cabeca = re.sub(r'^// (?:\[pt\] O resto do motor|The rest of the engine)[^\n]*\n|^include!\("motor/[^"]+"\);\n', '', cabeca, flags=re.M).rstrip('\n') + '\n\n'
    novo = cabeca + '// The rest of the engine, one file per source file of the game (stage 3 of the reorganisation, 07/11/2026).\n' \
        '// [pt] O resto do motor, um arquivo por arquivo de origem do jogo (etapa 3 da reorganização, 07/11/2026).\n' + \
        '\n'.join(inc) + '\n\n' + ''.join(fica)
    open(os.path.join(SRC, 'motor.rs'), 'w', encoding='utf-8').write(novo)
    print('gravado: %d arquivos, %d macros' % (len(plano), len(macros)))


if __name__ == '__main__':
    main()
