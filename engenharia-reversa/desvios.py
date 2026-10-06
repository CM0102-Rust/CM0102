r"""Analisa o inicio de rotinas do exe para desvio (hook) e gera a tabela usada pelo nucleo em Rust.

Uso: python desvios.py <exe> <va> [<va> ...]          ex.: desvios.py ..\jogo\cm0102.exe 90cfc0

Para cada rotina:
  - pega as primeiras instrucoes inteiras que somem >= 5 bytes (espaco do "jmp rel32");
  - confere que nenhuma delas e relativa ao endereco (jmp/call/jcc relativos, loop...): elas vao
    para o trampolim, em outro endereco, e mudariam de sentido;
  - confere que nenhum salto ou chamada do .text cai no meio desses bytes (so no inicio).
Imprime a entrada da tabela `rust/src/desvios.rs` (endereco + bytes esperados). Na abertura do jogo
o nucleo confere esses bytes antes de desviar e recusa se nao baterem.
"""
import csv
import os
import struct
import sys

import capstone

BASE = 0x400000
TEXTO = (0x1000, 0x567000)


def falha(msg):
    sys.exit('ERRO: ' + msg)


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    md.detail = True
    ini, fim = TEXTO
    mapa = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'docs', 'mapa_funcoes.csv')
    inicios = sorted(int(r['endereco'], 16) for r in csv.DictReader(open(mapa))) if os.path.exists(mapa) else []
    for va in (int(x, 16) for x in sys.argv[2:]):
        o = va - BASE
        prologo = []
        for ins in md.disasm(d[o:o + 32], va):
            prologo.append(ins)
            if sum(i.size for i in prologo) >= 5:
                break
        tam = sum(i.size for i in prologo)
        for i in prologo:
            relativa = i.group(capstone.CS_GRP_JUMP) or i.group(capstone.CS_GRP_CALL) or i.group(capstone.CS_GRP_BRANCH_RELATIVE)
            if relativa or 'eip' in i.op_str:
                falha('%#x: instrucao relativa no inicio (%s %s)' % (va, i.mnemonic, i.op_str))
        # Saltos/chamadas para dentro dos bytes copiados (exceto o proprio inicio).
        dentro = []
        for alvo in range(va + 1, va + tam):
            for op in (b'\xe8', b'\xe9'):
                p = d.find(op, ini, fim)
                while p != -1:
                    if p + 5 <= fim and struct.unpack_from('<i', d, p + 1)[0] + p + 5 + BASE == alvo:
                        dentro.append('%#x' % (p + BASE))
                    p = d.find(op, p + 1, fim)
        # Saltos curtos (rel8) por perto: desmonta linearmente a partir do inicio de funcao anterior
        # (pelo mapa), para so contar instrucoes de verdade e nao bytes soltos que parecem jcc.
        partida = max([f for f in inicios if f <= va - 130] or [va - 130])
        for x in md.disasm(d[partida - BASE:va - BASE + tam + 130], partida):
            if x.address >= va + tam + 130:
                break
            if x.size == 2 and (x.bytes[0] == 0xeb or 0x70 <= x.bytes[0] <= 0x7f):
                t = int(x.op_str, 16)
                if va < t < va + tam:
                    dentro.append('curto %#x' % x.address)
        if dentro:
            falha('%#x: ha saltos para dentro do prologo: %s' % (va, ', '.join(dentro)))
        texto = '; '.join('%s %s' % (i.mnemonic, i.op_str) for i in prologo)
        bs = ', '.join('0x%02x' % b for b in d[o:o + tam])
        print('// %#x: %s' % (va, texto))
        print('Prologo { endereco: %#x, bytes: &[%s] },' % (va, bs))


if __name__ == '__main__':
    main()
