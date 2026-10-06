"""Desmonta um trecho do .text, anotando chamadas a funcoes importadas e pushes de strings.

Uso: python desmonta.py <exe> <va_inicio> <va_fim>      (enderecos em hex, ex.: 413180 4132a0)
"""
import sys

import capstone
import pefile

BASE = 0x400000


def main():
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    exe, a, b = sys.argv[1], int(sys.argv[2], 16), int(sys.argv[3], 16)
    dados = open(exe, 'rb').read()
    pe = pefile.PE(data=dados, fast_load=True)
    pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
    iat = {'%#x' % imp.address: (imp.name or b'#').decode() for e in pe.DIRECTORY_ENTRY_IMPORT for imp in e.imports}

    def texto_em(va):
        o = va - BASE
        if 0 <= o < len(dados):
            fim = dados.find(b'\0', o, o + 100)
            s = dados[o:fim]
            if fim > o and all(32 <= c < 127 for c in s):
                return s.decode()

    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    for ins in md.disasm(dados[a - BASE:b - BASE], a):
        nota = next((' ; ' + nome for end, nome in iat.items() if end in ins.op_str), '')
        if not nota and ins.mnemonic == 'push' and ins.op_str.startswith('0x'):
            s = texto_em(int(ins.op_str, 16))
            if s:
                nota = ' ; "%s"' % s
        print('%08x  %-8s %s%s' % (ins.address, ins.mnemonic, ins.op_str, nota))


if __name__ == '__main__':
    main()
