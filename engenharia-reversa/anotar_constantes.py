r"""Anota o valor das constantes de ponto flutuante que o motor lê do exe (`read_f64_at(0x968e48)`): o comentário
`// 0x968e48 = 0.6` no fim da linha, lido do cm0102.exe 3.9.68 (etapa 4 da reorganização, achado do leitor de fora,
rodada 2). É só comentário: o código gerado não muda. Linha já anotada fica como está.

Uso: python anotar_constantes.py [--gravar]
"""
import os
import re
import struct
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
LEITURA = re.compile(r'read_f(64|32)_at\((0x[0-9a-f]+)\)')


def main():
    exe = open(os.path.join(R, 'jogo', 'cm0102.exe'), 'rb').read()

    def valor(bits, a):
        o = int(a, 16) - 0x400000  # .rdata/.data: deslocamento no arquivo = VA - 0x400000
        if not (0 <= o < len(exe) - 8):
            return None
        v = struct.unpack('<d', exe[o:o + 8])[0] if bits == '64' else struct.unpack('<f', exe[o:o + 4])[0]
        return ('%.10g' % v)

    n = 0
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(d, f)
            ls = open(p, encoding='utf-8').read().split('\n')
            mudou = False
            for i, l in enumerate(ls):
                cod, sep, com = l.partition('//')
                if cod.lstrip().startswith('///'):
                    continue
                achados = [(b, a) for b, a in LEITURA.findall(cod) if a not in com]
                if not achados:
                    continue
                notas = ', '.join('%s = %s' % (a, valor(b, a)) for b, a in dict.fromkeys(achados) if valor(b, a) is not None)
                if not notas:
                    continue
                ls[i] = (l + ' // ' + notas) if not sep else (l + '; ' + notas)
                mudou = True
                n += 1
            if mudou and '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8').write('\n'.join(ls))
    print('%d linhas anotadas' % n)


if __name__ == '__main__':
    main()
