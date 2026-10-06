r"""Candidatos a portar: funcoes-folha (sem call), pequenas e muito chamadas.

Uso: python alvos.py <exe> <mapa_funcoes.csv> [quantos=40] [tamanho_max=160]

Para cada funcao do mapa: tamanho, quantas chamadas diretas (call rel32) recebe, se chama algo
(call rel32, call [mem], call reg) e o arquivo-fonte. Lista as folhas por numero de chamadores.
"""
import csv
import struct
import sys

import capstone

BASE = 0x400000


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    quantos = int(sys.argv[3]) if len(sys.argv) > 3 else 40
    tam_max = int(sys.argv[4]) if len(sys.argv) > 4 else 160
    funcs = [(int(r['endereco'], 16), int(r['tamanho']), r['arquivo']) for r in csv.DictReader(open(sys.argv[2]))]
    chamadas = {}
    i = d.find(b'\xe8', 0x1000, 0x567000)
    while i != -1:
        alvo = struct.unpack_from('<i', d, i + 1)[0] + i + 5 + BASE
        chamadas[alvo] = chamadas.get(alvo, 0) + 1
        i = d.find(b'\xe8', i + 1, 0x567000)
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    linhas = []
    for va, tam, arq in funcs:
        n = chamadas.get(va, 0)
        if tam > tam_max or n < 20:
            continue
        o = va - BASE
        corpo = list(md.disasm(d[o:o + tam], va))
        # Corta no primeiro ret seguido de preenchimento (o tamanho do mapa pode incluir padding).
        ins = []
        for x in corpo:
            ins.append(x)
            if x.mnemonic == 'ret' and d[x.address + x.size - BASE] in (0x90, 0xcc):
                break
        if any(x.mnemonic == 'call' for x in ins):
            continue
        globais = sorted({x.op_str for x in ins if '[0x' in x.op_str})
        fp = any(x.mnemonic.startswith('f') for x in ins)
        linhas.append((n, va, sum(x.size for x in ins), len(ins), arq or '?', fp, len(globais)))
    linhas.sort(reverse=True)
    print('%8s  %-10s %5s %4s  %-3s %-4s  %s' % ('chamadas', 'endereco', 'bytes', 'ins', 'x87', 'glob', 'arquivo'))
    for n, va, b, k, arq, fp, g in linhas[:quantos]:
        print('%8d  %#-10x %5d %4d  %-3s %-4d  %s' % (n, va, b, k, 'sim' if fp else '', g, arq))


if __name__ == '__main__':
    main()
