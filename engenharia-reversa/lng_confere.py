"""Confere um arquivo de nomes traduzidos (.lng) com o banco de uma pasta Data, sem abrir o jogo.

database_load_language_names (0x5339a0, Database.cpp) lê 8 u32 no início do .lng e compara seis deles com as
quantidades do banco carregado (0x533e6a..0x533ec5); se um não bate, mostra "v3.9.68 Database..cpp 7784", zera
[0xb67a34] e sai sem ler nome nenhum (o jogo fica com os nomes do banco).

Uso: python lng_confere.py <arquivo.lng> <pasta Data>
"""
import os
import struct
import sys

# (nome, campo do cabeçalho, arquivo do banco, tamanho do registro, global que a original compara)
CAMPOS = [
    ('continentes', 1, 'continent.dat', 198, '[0xae234c]'),
    ('nações', 3, 'nation.dat', 290, '[0xae2350]'),
    ('cidades', 4, 'city.dat', 56, '[0xae2354]'),
    ('estádios', 6, 'stadium.dat', 78, '[0xae2358]'),
    ('competições de pessoal', 7, 'staff_comp.dat', 101, '[0xae2388]'),
]


def contar(data, arquivo, registro):
    return os.path.getsize(os.path.join(data, arquivo)) // registro


def conferir(lng, data):
    """[(nome, .lng, banco, igual)] e os clubes do .lng (só informativo: a original não os compara)."""
    cab = struct.unpack('<8I', open(lng, 'rb').read(32))
    if cab[0] == 0xf1fffff1:
        sys.exit('formato com a marca 0xf1fffff1 (bytes trocados): não suportado aqui')
    r = [(nome, cab[i], contar(data, arq, reg), cab[i] == contar(data, arq, reg)) for nome, i, arq, reg, _ in CAMPOS]
    # a quarta conferência: competições de clubes + de seleções == [0xae2378] - 127 (os 127 amistosos)
    comps = contar(data, 'club_comp.dat', 107) + contar(data, 'nation_comp.dat', 107)
    r.insert(3, ('competições (clubes + seleções)', cab[5] + cab[2], comps, cab[5] + cab[2] == comps))
    return r, cab[0]


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    r, clubes = conferir(sys.argv[1], sys.argv[2])
    for nome, a, b, ok in r:
        print('%-34s .lng %6d   banco %6d   %s' % (nome, a, b, 'ok' if ok else 'DIFERENTE'))
    print('%-34s .lng %6d   (não conferido pelo jogo)' % ('clubes', clubes))
    print('o jogo aceita este .lng' if all(x[3] for x in r) else 'o jogo recusa este .lng (Database.cpp 7784)')


if __name__ == '__main__':
    main()
