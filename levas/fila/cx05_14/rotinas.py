# Os dados proprios da Eurocopa, da minicopa e da minliga gravados no jogo salvo (euro_champ.cpp, mini_cup.cpp,
# mini_league.cpp; leitor cx05, rodada 5, 24/11/2026). Item do SAVE: a gravacao do arquivo temporario de cada
# competicao (League::save_to_temp_file 0x6891c0 / Cup::save_cup_to_disk 0x5210f0) chama a virtual 0x24 com o FILE
# aberto em CM3_TEMP; os .tmp vao para o .qst. A Eurocopa existe em todo jogo (a de 2004): roda em todo save; a minicopa
# e a minliga so com torneios de pre-temporada vivos.
# - 0x57c5d0 EuroChamp::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (4 pushes; add esp -> cdecl) da lista
#   semeada (+0xee, 16 x 5), de +0x13e e +0x140 (2 bytes); devolve o eax do ultimo fwrite. Prologo push esi; push edi;
#   mov edi, [esp+0xc] (6 bytes).
# - 0x7664c0 MiniCup::save_extra_data thiscall(arquivo) ret 4: +0xb2 (8) e +0xba (2). Mesmo prologo.
# - 0x766d90 MiniLeague::save_extra_data thiscall(arquivo) ret 4: +0xee (8). Prologo mov eax, [esp+4]; add ecx, 0xee
#   (10 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (gravam o save); a prova e a do save.
SAVE = True
ROTINAS = [('euro_champ_save_extra_data', 0x57c5d0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('mini_cup_save_extra_data', 0x7664c0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('mini_league_save_extra_data', 0x766d90, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00])]
DOCS = ('os dados próprios da Eurocopa e dos torneios de amistosos no jogo salvo', 'the European Championship and friendly tournaments\' own data in the saved game',
        [('euro_champ_save_extra_data', 'EuroChamp::save_extra_data', '0x57c5d0', 'euro_champ.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Eurocopa a lista semeada das 16 seleções e a data da semeadura, na ordem da leitura',
          'writes into the European Championship temporary file the seeded list of the 16 nations and the seeding date, in the order the load reads them'),
         ('mini_cup_save_extra_data', 'MiniCup::save_extra_data', '0x7664c0', 'mini_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da minicopa a data de início e a palavra das regras de desempate',
          'writes into the mini cup temporary file the start date and the tie rules word'),
         ('mini_league_save_extra_data', 'MiniLeague::save_extra_data', '0x766d90', 'mini_league.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da minliga a data de início',
          'writes into the mini league temporary file the start date')])
CAMPOS = []
INTERESSA = []
