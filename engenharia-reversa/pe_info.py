"""Cabecalho PE: MD5, versao do linker, data de compilacao, secoes e funcoes importadas.

Uso: python pe_info.py <exe> [<exe> ...]
"""
import datetime
import hashlib
import sys

import pefile


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    for f in sys.argv[1:]:
        d = open(f, 'rb').read()
        pe = pefile.PE(data=d)
        oh = pe.OPTIONAL_HEADER
        data = datetime.datetime.fromtimestamp(pe.FILE_HEADER.TimeDateStamp, datetime.timezone.utc)
        print('=====', f, len(d), 'bytes, md5', hashlib.md5(d).hexdigest())
        print('linker %d.%d, compilado %s, imagebase %#x, entrada %#x' % (
            oh.MajorLinkerVersion, oh.MinorLinkerVersion, data.isoformat(), oh.ImageBase, oh.ImageBase + oh.AddressOfEntryPoint))
        for s in pe.sections:
            print('  secao %-8s va %#08x tam %#08x  arquivo %#08x tam %#08x' % (
                s.Name.rstrip(b'\0').decode(), oh.ImageBase + s.VirtualAddress, s.Misc_VirtualSize, s.PointerToRawData, s.SizeOfRawData))
        for e in pe.DIRECTORY_ENTRY_IMPORT:
            print('  %s: %s' % (e.dll.decode(), ', '.join('%s@%#x' % ((i.name or b'#').decode(), i.address) for i in e.imports)))


if __name__ == '__main__':
    main()
