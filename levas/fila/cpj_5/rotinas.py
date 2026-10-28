# Humanos e olheiros: limpar as partidas assistidas que os técnicos humanos não podem mais ver
# (HUMAN_MANAGER::clear_invalid_watched_fixtures 0x5fe8d0, chamada pelo interesse do dia 0x6b5040, já em Rust; a
# 0x5fe700 decide, original), os alvos dos pedidos de observação de um clube (SCOUT_MANAGER::collect_club_scout_targets
# 0x7f9690, pela 0x7b5be0 da busca; 0x68d080 esvazia e 0x68d0d0 acrescenta) e trocar o olheiro da busca
# (PLAYER_SEARCH::set_scout 0x7ae8a0, pelas 0x7f7270/0x8043c0; o olheiro atual pela 0x7ae950 da 244ª; 0x7e4920 grava os
# ids). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio.
# Comparadores: lado a lado pelas partidas assistidas e pelo olheiro; os alvos pela lista que a original encheu.
SAVE = False
ROTINAS = [('limpar_partidas_assistidas', 0x5fe8d0, [0x51, 0x53, 0x55, 0x56, 0x33, 0xdb]),
           ('alvos_dos_olheiros_do_clube', 0x7f9690, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('trocar_olheiro_da_busca', 0x7ae8a0, [0x53, 0x56, 0x8b, 0xf1, 0x83, 0xcb, 0xff])]
DOCS = ('limpar as partidas assistidas que os humanos não podem ver, os alvos dos olheiros do clube e trocar o olheiro da busca',
        'clearing the watched fixtures the humans may not watch, the club scout targets and changing the search scout',
        [('limpar_partidas_assistidas', 'HUMAN_MANAGER::clear_invalid_watched_fixtures', '0x5fe8d0', 'human_manager.cpp', 'thiscall',
          'zera (79 bytes) a partida assistida de cada técnico humano ativo que não pode mais vê-la (0x5fe700); devolve 1',
          'clears (79 bytes) the watched fixture of each active human manager who may no longer watch it (0x5fe700); returns 1'),
         ('alvos_dos_olheiros_do_clube', 'SCOUT_MANAGER::collect_club_scout_targets', '0x7f9690', 'scout_manager.cpp', 'thiscall, ret 8',
          'esvazia a lista e acrescenta a pessoa de cada pedido de observação do clube; devolve se há algum',
          'empties the list and adds the person of each scout request of the club; returns whether there is any'),
         ('trocar_olheiro_da_busca', 'PLAYER_SEARCH::set_scout', '0x7ae8a0', 'player_search.cpp', 'thiscall, ret 4',
          'olheiro novo: grava os ids (0x7e4920) ou −1, e as missões de nação, região e competição voltam a −1; devolve 1',
          'new scout: stores the ids (0x7e4920) or −1, and the nation, region and competition assignments go back to −1; returns 1')])
CAMPOS = [('humanos [this] + 0x9ad4 partidas assistidas (79 bytes, 0xa6ef por humano); pessoa +0x5f ativo; olheiros +0x5c pedidos (0x11 bytes: clube, pessoa), +0x64 quantos; PLAYER_SEARCH +0x174..+0x184 olheiro, +0x18c, +0x194, +0x195 missões; escreve (limpar_partidas_assistidas, alvos_dos_olheiros_do_clube, trocar_olheiro_da_busca)',
           'humans [this] + 0x9ad4 watched fixtures (79 bytes, 0xa6ef per human); person +0x5f active; scouts +0x5c requests (0x11 bytes: club, person), +0x64 count; PLAYER_SEARCH +0x174..+0x184 scout, +0x18c, +0x194, +0x195 assignments; writes (limpar_partidas_assistidas, alvos_dos_olheiros_do_clube, trocar_olheiro_da_busca)')]
INTERESSA = ['SGE']
