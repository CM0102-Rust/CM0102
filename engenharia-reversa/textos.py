"""Lista strings ASCII do exe que casam com uma expressao regular (sem diferenciar maiusculas).

Uso: python textos.py <exe> <regex>
Mostra deslocamento no arquivo e VA (validos para .text/.rdata/.data, onde VA = deslocamento + 0x400000).
"""
import re
import sys


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    pad = re.compile(sys.argv[2].encode(), re.I)
    for m in re.finditer(rb'[\x20-\x7e]{4,}', d):
        if pad.search(m.group()):
            print('%#08x  va %#08x  %s' % (m.start(), m.start() + 0x400000, m.group().decode()))


if __name__ == '__main__':
    main()
