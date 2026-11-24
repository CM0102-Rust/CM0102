# Os dados proprios da Eurocopa, da minicopa e da minliga lidos do jogo salvo (euro_champ.cpp, mini_cup.cpp,
# mini_league.cpp; leitor cx05, rodada 5, 24/11/2026). Item do SAVE (CARGA): a carga do arquivo temporario de cada
# competicao (League::load_from_temp_file 0x687b10 / Cup::load_from_disk 0x51fc00) chama a virtual 0x20 com o FILE
# aberto. A Eurocopa roda em toda carga; a minicopa e a minliga so com torneios vivos no save.
# - 0x57c480 EuroChamp::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (cdecl) de 16 x 5 em +0xee
#   (curto: a caixa da linha 0x404 e 0), de +0x13e (0x40c) e de +0x140 (0x413). Prologo sub esp, 0x200 (6 bytes).
# - 0x766380 MiniCup::load_extra_data thiscall(arquivo) ret 4 -> 1: +0xb2 (8) e +0xba (2); curto: a caixa (0x13b,
#   0x143), game_fatal_cleanup 0x5cc7d0, 0x5a8f60 e exit(-1) 0x945934 (fatal_error_at). Prologo sub esp, 0x200.
# - 0x766cf0 MiniLeague::load_extra_data thiscall(arquivo) ret 4 -> 0/1: +0xee (8); curto: a caixa (0x116) e 0.
#   Prologo mov eax, [esp+4]; sub esp, 0x200 (10 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx02_13 (compare_cup_extra_data_load): integrar depois dele.
# Comparadores: lado a lado com o arquivo de volta a posicao (o mesmo trecho lido duas vezes), os campos, o retorno e a
# posicao final.
SAVE = True
CARGA = True
ROTINAS = [('euro_champ_load_extra_data', 0x57c480, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('mini_cup_load_extra_data', 0x766380, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('mini_league_load_extra_data', 0x766cf0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os dados próprios da Eurocopa e dos torneios de amistosos lidos do jogo salvo', 'the European Championship and friendly tournaments\' own data read from the saved game',
        [('euro_champ_load_extra_data', 'EuroChamp::load_extra_data', '0x57c480', 'euro_champ.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Eurocopa a lista semeada das 16 seleções e a data da semeadura; leitura curta: a caixa de erro e 0',
          'reads from the European Championship temporary file the seeded list of the 16 nations and the seeding date; a short read: the error box and 0'),
         ('mini_cup_load_extra_data', 'MiniCup::load_extra_data', '0x766380', 'mini_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da minicopa a data de início e a palavra das regras de desempate; leitura curta: a caixa de erro e o jogo termina',
          'reads from the mini cup temporary file the start date and the tie rules word; a short read: the error box and the game ends'),
         ('mini_league_load_extra_data', 'MiniLeague::load_extra_data', '0x766cf0', 'mini_league.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da minliga a data de início; leitura curta: a caixa de erro e 0',
          'reads from the mini league temporary file the start date; a short read: the error box and 0')])
CAMPOS = []
INTERESSA = []
