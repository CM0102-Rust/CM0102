"""Confere os prologos do motor.rs: instrucoes inteiras e sem salto/chamada relativa (o trampolim copia os bytes como estao)
e nenhum endereco repetido (com o date.rs).

Uso: python prologos.py"""
import re
from capstone import Cs, CS_ARCH_X86, CS_MODE_32
d = open(r'D:\Sistemas\CM0102\jogo\cm0102.exe', 'rb').read()
# entrada só do 1080 (o prólogo não é o do original; 0x603834): desmonta do cm0102-1080.exe
d1080 = open(r'D:\Sistemas\CM0102\jogo\cm0102-1080.exe', 'rb').read()
src = open(r'D:\Sistemas\CM0102\rust\src\motor.rs', encoding='utf-8').read()
md = Cs(CS_ARCH_X86, CS_MODE_32)
ruins = 0
# prologos que terminam numa instrucao relativa refeita pelo trampolim (RELATIVAS em detours.rs)
des = open(r'D:\Sistemas\CM0102\rust\src\detours.rs', encoding='utf-8').read()
relativas = {int(a, 16): int(k) for a, k in re.findall(r'\((0x[0-9a-f]+), (\d+)\)', des.split('pub const RELATIVAS')[1].split(';')[0])}
for a, b in re.findall(r'Prologue \{ address: (0x[0-9a-f]+), bytes: &\[([^\]]*)\]', src):
    a = int(a, 16); n = len(b.split(','))
    fim = a
    bs = bytes(int(x, 16) for x in b.split(','))
    exe = d if d[a - 0x400000:a - 0x400000 + n] == bs or d1080[a - 0x400000:a - 0x400000 + n] != bs else d1080
    for ins in md.disasm(exe[a - 0x400000:a - 0x400000 + n + 15], a):
        if fim >= a + n:
            break
        rel_ok = relativas.get(a) == ins.address - a and (ins.mnemonic == 'call' or (ins.mnemonic.startswith('j') and ins.size == 2 and ins.mnemonic != 'jmp'))
        if (ins.mnemonic.startswith('j') or ins.mnemonic in ('call', 'loop', 'jecxz')) and not rel_ok:
            print('RUIM salto: %#x %s %s' % (ins.address, ins.mnemonic, ins.op_str)); ruins += 1
        fim = ins.address + ins.size
    if fim != a + n:
        print('RUIM corte: %#x' % a); ruins += 1
# o mesmo endereco em dois prologos (motor.rs e date.rs): o segundo desvio e recusado e a rotina
# nova nunca roda (aconteceu com 0x6d3de0/0x6d3610, ja portadas)
todos = re.findall(r'address: (0x[0-9a-f]+)', src + open(r'D:\Sistemas\CM0102\rust\src\date.rs', encoding='utf-8').read())
for a in sorted({x for x in todos if todos.count(x) > 1}):
    print('RUIM repetido: %s ja tem prologo' % a); ruins += 1
print('prologos com problema:', ruins)
