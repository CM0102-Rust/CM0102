"""Confere os prologos do motor.rs: instrucoes inteiras e sem salto/chamada relativa (o trampolim copia os bytes como estao).

Uso: python prologos.py"""
import re
from capstone import Cs, CS_ARCH_X86, CS_MODE_32
d = open(r'D:\Sistemas\CM0102\jogo\cm0102.exe', 'rb').read()
src = open(r'D:\Sistemas\CM0102\rust\src\motor.rs', encoding='utf-8').read()
md = Cs(CS_ARCH_X86, CS_MODE_32)
ruins = 0
for a, b in re.findall(r'Prologo \{ endereco: (0x[0-9a-f]+), bytes: &\[([^\]]*)\]', src):
    a = int(a, 16); n = len(b.split(','))
    fim = a
    for ins in md.disasm(d[a - 0x400000:a - 0x400000 + n + 15], a):
        if fim >= a + n:
            break
        if ins.mnemonic.startswith('j') or ins.mnemonic in ('call', 'loop', 'jecxz'):
            print('RUIM salto: %#x %s %s' % (ins.address, ins.mnemonic, ins.op_str)); ruins += 1
        fim = ins.address + ins.size
    if fim != a + n:
        print('RUIM corte: %#x' % a); ruins += 1
print('prologos com problema:', ruins)
