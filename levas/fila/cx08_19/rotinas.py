# Os dados proprios da Segunda Divisao belga no jogo salvo (bel_second.cpp; leitor cx08, rodada 5, 24/11/2026). Item do
# SAVE (CARGA). Rodam na gravacao e na carga do arquivo temporario da liga (as virtuais 0x20 e 0x24 chamadas pela
# League); so com um save que tenha a Belgica: o FeriasTesteSave (so a Inglaterra) nao as chama. A prova pede um save com
# a Belgica.
# - 0x420e30 BelSecond::load_extra_data thiscall(arquivo) ret 4 -> 1: fread 0x945bfe (cdecl) de +0xf6, +0xf7, +0xf8
#   (1 x 1 cada) e +0xee (4 x 2), sem conferir. Prologo push esi; push edi; mov edi, [esp+0xc] (6 bytes).
# - 0x420e90 BelSecond::save_extra_data thiscall(arquivo) ret 4: o mesmo com fwrite 0x945ce6; devolve o eax do ultimo
#   fwrite. Prologo igual.
# Comparadores: a carga lado a lado (compare_cup_extra_data_load do cx02_13: a original le, o arquivo volta e a copia le
# de novo); a gravacao so a original (a prova do save confere).
# Depende do cx08_16 (BEL_SECOND_PERIOD_WON) e do cx02_13 (compare_cup_extra_data_load): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = True
CARGA = True
ROTINAS = [('bel_second_load_extra_data', 0x420e30, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('bel_second_save_extra_data', 0x420e90, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os dados próprios da Segunda Divisão belga no jogo salvo', "the Belgian Second Division's own data in the saved game",
        [('bel_second_load_extra_data', 'BelSecond::load_extra_data', '0x420e30', 'bel_second.cpp', 'thiscall, ret 4',
          'lê do jogo salvo as três marcas de campeão de período e os dois números de +0xee da Segunda Divisão belga; devolve 1 sem conferir a leitura',
          'reads back from the saved game the three period winner flags and the two dwords at +0xee of the Belgian Second Division; returns 1 without checking the reads'),
         ('bel_second_save_extra_data', 'BelSecond::save_extra_data', '0x420e90', 'bel_second.cpp', 'thiscall, ret 4',
          'grava no jogo salvo as três marcas de campeão de período e os dois números de +0xee da Segunda Divisão belga',
          'writes to the saved game the three period winner flags and the two dwords at +0xee of the Belgian Second Division')])
CAMPOS = []
INTERESSA = []
