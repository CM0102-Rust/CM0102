"""Os ids-chave do exe: nações, competições, clubes, prêmios e cores que o código trata pelo nome.

A carga (set_key_ids 0x601a40) põe −1 em cada global g_key_*, procura cada nome fixo nas tabelas do banco
e guarda ali a linha achada; as rotinas depois comparam o id do registro com essa global. Esta ferramenta
cruza os nomes das globais (mapa de maxevans60, CC BY 4.0) com as referências no código do exe e diz,
para cada chave, se a carga a confere (falta = caixa "Unable to find the %s index") e em que arquivos
do código-fonte original ela é usada fora da carga. Ver docs/CHAVES_DO_EXE.md.

Uso: python chaves.py [<exe>] [--csv <saida>]     (padrão: ../jogo/cm0102.exe, ../docs/chaves_uso.csv)
"""
import bisect
import collections
import csv
import os
import re
import struct
import sys

BASE = 0x400000
TEXTO = (0x1000, 0x567000)               # .text do cm0102.exe
CARGA = (0x601a40, 0x60ba80)             # set_key_ids
NOMES = (0x60d810, 0x617000)             # match_*_key_id: as cadeias de nomes que gravam as globais
RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PRINCIPAL = RAIZ.split(os.sep + '.claude')[0]   # jogo/ e referencias/ ficam na cópia principal (fora do git)
MAPA = os.path.join(PRINCIPAL, 'referencias', 'cm0102-function-map', 'data-map', 'globals.tsv')


def tipo(nome):
    p = nome.split('_')[2:]
    return p[0] + ('_comp' if len(p) > 1 and p[1] == 'comp' else '')


def arquivos():
    fun = []
    with open(os.path.join(RAIZ, 'docs', 'mapa_funcoes.csv'), encoding='utf-8') as f:
        for x in csv.DictReader(f):
            fun.append((int(x['endereco'], 16), int(x['tamanho']), x['arquivo'].split('/')[-1]))
    fun.sort()
    ini = [f[0] for f in fun]

    def arq(va):
        i = bisect.bisect_right(ini, va) - 1
        return (fun[i][2] or '?') if i >= 0 and va < fun[i][0] + fun[i][1] else '?'
    return arq


def main():
    args = sys.argv[1:]
    saida = os.path.join(RAIZ, 'docs', 'chaves_uso.csv')
    if '--csv' in args:
        k = args.index('--csv'); saida = args[k + 1]; del args[k:k + 2]
    exe = args[0] if args else os.path.join(PRINCIPAL, 'jogo', 'cm0102.exe')
    d = open(exe, 'rb').read()
    with open(MAPA, encoding='utf-8') as f:
        chaves = {int(x['address'], 16): x['name'] for x in csv.DictReader(f, delimiter='\t')
                  if x['name'].startswith('g_key_') and not re.search('loop_end|active_count', x['name'])}  # contadores, não chaves
    arq = arquivos()
    linhas, resumo = [], collections.defaultdict(lambda: [0, 0, 0, 0, set()])
    for va, nome in sorted(chaves.items()):
        agulha = struct.pack('<I', va)
        usos, conferida = [], False
        i = d.find(agulha, *TEXTO)
        while i != -1:
            ref = i + BASE
            if CARGA[0] <= ref < CARGA[1]:
                conferida |= d[i - 2:i] == b'\x39\x3d'      # cmp [chave], edi (edi = −1) antes do aviso de falta
            elif not NOMES[0] <= ref < NOMES[1]:
                usos.append(ref)
            i = d.find(agulha, i + 1, TEXTO[1])
        arqs = sorted(collections.Counter(arq(u) for u in usos).items(), key=lambda x: -x[1])
        linhas.append((nome, '%#x' % va, tipo(nome), 'sim' if conferida else 'não', len(usos),
                       ' '.join('%s:%d' % a for a in arqs)))
        r = resumo[tipo(nome)]
        r[0] += 1; r[1] += conferida; r[2] += bool(usos); r[3] += len(usos); r[4] |= {a for a, _ in arqs}
    with open(saida, 'w', encoding='utf-8', newline='') as f:
        w = csv.writer(f)
        w.writerow(['chave', 'endereco', 'tipo', 'conferida_na_carga', 'usos', 'arquivos'])
        w.writerows(linhas)
    print('%-12s %6s %10s %7s %6s %9s' % ('tipo', 'chaves', 'conferidas', 'usadas', 'usos', 'arquivos'))
    for t, r in sorted(resumo.items(), key=lambda x: -x[1][0]):
        print('%-12s %6d %10d %7d %6d %9d' % (t, r[0], r[1], r[2], r[3], len(r[4])))
    print('gravado', saida)


if __name__ == '__main__':
    main()
