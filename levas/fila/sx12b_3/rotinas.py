# Os construtores do hall da fama, de uma entrada dele e de uma nacao-chave (hall_of_fame.cpp, key_nation.cpp; leitor
# sx12b, rodada 5, 23/11/2026). Rodam no inicio de cada execucao do jogo (tambem na carreira de teste):
# - HallOfFame::HallOfFame 0x5ef0b0: chamado uma vez por 0x5cc773 (inicio do programa);
# - HallOfFameEntry::HallOfFameEntry 0x5ef120: pelo iterador de vetor do inicializador estatico 0x5efd00 (2800, o
#   buffer de importacao 0xb0be38) e do construtor acima (2800);
# - KEY_NATION::KEY_NATION 0x667100: pelo iterador do inicializador estatico 0x6670b0 (35, a tabela 0xb63d60).
# Os desvios ja estao no lugar nessa hora (a DLL instala no DllMain, antes do _initterm do exe).
# Convencoes conferidas na desmontagem:
# - 0x5ef0b0: thiscall(this), ret 0, eax this; quadro SEH so pelo iterador; iterador 0x9463af stdcall(this, 0x78, 0xaf0,
#   0x5ef120, 0x5a53c0); rep stosd 0x14820 dwords; HallOfFame::load 0x5efaa0 thiscall (sx12_7) e, com 0,
#   HallOfFame::save 0x5f00c0 thiscall.
# - 0x5ef120: thiscall(this), ret 0, eax this; Date::Date 0x54a110 thiscall(this+0x70).
# - 0x667100: thiscall(this), ret 0, eax this; quadro SEH; Date::Date em +0x2a e em +0x39.
# Comparadores lado a lado: o hall da fama inteiro (0x52080 bytes; a carga le o mesmo arquivo duas vezes; se falhar, a
# original grava um novo e a copia o le: a tabela e a mesma), a entrada (0x78) e a nacao-chave (0x48).
SAVE = False
ROTINAS = [('construct_hall_of_fame', 0x5ef0b0, [0x6a, 0xff, 0x68, 0x06, 0xac, 0x95, 0x00]),
           ('construct_hall_of_fame_entry', 0x5ef120, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x70]),
           ('construct_key_nation', 0x667100, [0x6a, 0xff, 0x68, 0x2b, 0xc3, 0x95, 0x00])]
DOCS = ('os construtores do hall da fama e das nações-chave',
        'the hall of fame and key nation constructors',
        [('construct_hall_of_fame', 'HallOfFame::HallOfFame', '0x5ef0b0', 'hall_of_fame.cpp', 'thiscall',
          'monta o hall da fama: as 2800 entradas (28 tabelas de 100) zeradas, depois lidas do hall_of_fame.bin; se a leitura falhar (sem o arquivo ou com ele estragado), grava um arquivo novo com as tabelas zeradas',
          'builds the hall of fame: the 2800 entries (28 tables of 100) cleared, then read from hall_of_fame.bin; when the read fails (no file or a bad one), writes a new file with the cleared tables'),
         ('construct_hall_of_fame_entry', 'HallOfFameEntry::HallOfFameEntry', '0x5ef120', 'hall_of_fame.cpp', 'thiscall',
          'monta uma entrada do hall da fama: só a data, vazia',
          'builds one hall of fame entry: only its date, empty'),
         ('construct_key_nation', 'KEY_NATION::KEY_NATION', '0x667100', 'key_nation.cpp', 'thiscall',
          'monta uma linha da tabela das nações-chave: só as datas de início e fim da temporada, vazias',
          'builds one row of the key nation table: only the season start and end dates, empty')])
