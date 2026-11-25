# Nove destrutores escalares de competicoes (kor_super_cup.cpp, mini_cup.cpp, mini_league.cpp, nir_charity.cpp,
# nir_cup.cpp, nir_lge_cup.cpp, nor_cup.cpp, uefa_cup.cpp, world_club_cup.cpp; leitor rx15, rodada 6, 25/11/2026).
# Item de TELAS, como o cx03_3 e o cx12_1: o slot 0 da vtable de cada classe (0x96cf38, 0x96d8ac, 0x96d94c, 0x96da24,
# 0x96dac4, 0x96dc18, 0x96dd80, 0x970790, 0x971250; a unica referencia de cada um), chamado com 1 quando o jogo e
# desmontado (carregar outro save com um jogo aberto, ou sair da carreira); a minicopa e a minliga tambem quando um
# torneio de pre-temporada e cancelado (0 chamadas na carreira ENG ate 2002-06-01, ver o cx05_1). Caminho no jogo: com
# uma carreira aberta que tenha essas competicoes (a Copa da UEFA e o Mundial de Clubes sempre; a Coreia, a Irlanda do
# Norte e a Noruega com a liga da nacao carregada; a minicopa e a minliga com torneios de pre-temporada vivos, julho),
# carregar outro save, jogando comparando.
# Corpo: push esi; mov esi, ecx; call ~X (thiscall, ret 0); test byte [esp+8], 1; je; push esi; call operator delete
# 0x944c94 (cdecl); mov eax, esi; ret 4. O prologo tem 3 bytes antes de um call rel32: entra pelas RELATIVAS (endereco,
# 3), ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo endereco (passa pelo desvio quando portado: 0x765ea0
# e 0x7668f0 no cx05_1).
# Depende do cx03_3 (delete_through): integrar depois dele.
# Iguais no cm0102-1080.exe (as nove inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('delete_kor_super_cup', 0x66d9e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_mini_cup', 0x765e10, [0x56, 0x8b, 0xf1, 0xe8, 0x88, 0x00, 0x00, 0x00]),
           ('delete_mini_league', 0x7667e0, [0x56, 0x8b, 0xf1, 0xe8, 0x08, 0x01, 0x00, 0x00]),
           ('delete_nir_charity', 0x789fb0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_nir_cup', 0x78a780, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_nir_lge_cup', 0x78c9d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_nor_cup', 0x78f1f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_uefa_cup', 0x9024d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_world_club_cup', 0x92b680, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('nove destrutores escalares de competições (Coreia, minicopa e minliga, Irlanda do Norte, Noruega, UEFA e Mundial de Clubes)',
        'nine competition scalar deleting destructors (Korea, mini cup and mini league, Northern Ireland, Norway, UEFA and the Club World Cup)',
        [('delete_kor_super_cup', 'KorSuperCup::scalar_deleting_destructor', '0x66d9e0', 'kor_super_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Coreia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Korean Super Cup and, with bit 0, deletes the object'),
         ('delete_mini_cup', 'MiniCup::scalar_deleting_destructor', '0x765e10', 'mini_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da minicopa de pré-temporada e, com o bit 0, apaga o objeto',
          'runs the destructor of the pre-season mini cup and, with bit 0, deletes the object'),
         ('delete_mini_league', 'MiniLeague::scalar_deleting_destructor', '0x7667e0', 'mini_league.cpp', 'thiscall, ret 4',
          'roda o destrutor da minliga de pré-temporada e, com o bit 0, apaga o objeto',
          'runs the destructor of the pre-season mini league and, with bit 0, deletes the object'),
         ('delete_nir_charity', 'NirCharity::scalar_deleting_destructor', '0x789fb0', 'nir_charity.cpp', 'thiscall, ret 4',
          'roda o destrutor da Charity Shield da Irlanda do Norte e, com o bit 0, apaga o objeto',
          'runs the destructor of the Northern Irish Charity Shield and, with bit 0, deletes the object'),
         ('delete_nir_cup', 'NirCup::scalar_deleting_destructor', '0x78a780', 'nir_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Irlanda do Norte e, com o bit 0, apaga o objeto',
          'runs the destructor of the Northern Irish Cup and, with bit 0, deletes the object'),
         ('delete_nir_lge_cup', 'NirLgeCup::scalar_deleting_destructor', '0x78c9d0', 'nir_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga da Irlanda do Norte e, com o bit 0, apaga o objeto',
          'runs the destructor of the Northern Irish League Cup and, with bit 0, deletes the object'),
         ('delete_nor_cup', 'NorCup::scalar_deleting_destructor', '0x78f1f0', 'nor_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Noruega e, com o bit 0, apaga o objeto',
          'runs the destructor of the Norwegian Cup and, with bit 0, deletes the object'),
         ('delete_uefa_cup', 'UefaCup::scalar_deleting_destructor', '0x9024d0', 'uefa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da UEFA e, com o bit 0, apaga o objeto',
          'runs the destructor of the UEFA Cup and, with bit 0, deletes the object'),
         ('delete_world_club_cup', 'WorldClubCup::scalar_deleting_destructor', '0x92b680', 'world_club_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Mundial de Clubes da FIFA e, com o bit 0, apaga o objeto',
          'runs the destructor of the FIFA Club World Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
