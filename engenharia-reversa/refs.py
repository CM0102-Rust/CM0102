"""Quem usa um endereco: referencias absolutas (push/mov/call [iat]) e chamadas/saltos relativos (E8/E9).

Uso: python refs.py <exe> <va> [<va> ...]      (enderecos em hex)
"""
import struct
import sys

BASE = 0x400000
TEXTO = (0x1000, 0x567000)  # .text do cm0102.exe (o GDI termina antes; a folga nao atrapalha)


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    ini, fim = TEXTO
    for va in (int(x, 16) for x in sys.argv[2:]):
        abs_, rel = [], []
        agulha = struct.pack('<I', va)
        i = d.find(agulha, ini, fim)
        while i != -1:
            abs_.append('%#x' % (i + BASE))
            i = d.find(agulha, i + 1, fim)
        for op, nome in ((0xe8, 'call'), (0xe9, 'jmp')):
            i = d.find(bytes([op]), ini, fim)
            while i != -1:
                if i + 5 <= fim and struct.unpack_from('<i', d, i + 1)[0] + i + 5 + BASE == va:
                    rel.append('%s %#x' % (nome, i + BASE))
                i = d.find(bytes([op]), i + 1, fim)
        print('%#x' % va)
        print('  absolutas:', ' '.join(abs_) or '-')
        print('  relativas:', ' '.join(rel) or '-')


if __name__ == '__main__':
    main()
