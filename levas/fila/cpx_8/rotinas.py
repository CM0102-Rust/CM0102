# A tática ativa do clube (tactics.cpp, human_manager.cpp): Tactics::get_club_tactic 0x896660 (pela formação de cada
# jogo 0x6e0230, pelas fraquezas do jogador 0x5470d0, pela mídia e pelas telas), HUMAN_MANAGER::select_tactic_slot
# 0x5fbe20 e HUMAN_MANAGER::get_active_tactic 0x5fbd00 (as duas pela 0x896660 no clube humano, pela escalação 0x6db500,
# pela formação 0x6e0230, pelo dia de jogo 0x6b4360 e pelas telas). Iguais no cm0102-1080.exe; sem SEH; sem x87; o
# quadro de 0x200 bytes é só da caixa de erro (a original). Comparadores lado a lado: a 0x896660 pela devolução e pelo
# byte da vaga ativa do técnico humano do clube (a 0x5fbe20 grava), a 0x5fbe20 pelo byte da vaga, a 0x5fbd00 pela
# devolução; os casos de caixa ("Error") só pela original.
SAVE = False
ROTINAS = [('tatica_do_clube', 0x896660, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escolher_vaga_de_tatica', 0x5fbe20, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tatica_ativa_do_humano', 0x5fbd00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele',
        'the active club tactic: the tactic of the club team, the human manager tactic slot and his active tactic',
        [('tatica_do_clube', 'Tactics::get_club_tactic', '0x896660', 'tactics.cpp', 'thiscall, ret 8',
          'no clube humano (sem +0x9a01 ou com a opção 2 das férias), a vaga pelo time e o bloco ativo do técnico; nos outros, pelo índice na linha do clube, o bloco pronto (abaixo da conta das prontas) ou o próprio',
          'for a human club (without +0x9a01 or with holiday option 2), the slot for the team and the manager active block; otherwise, by the index in the club row, the preset block (below the preset count) or the own one'),
         ('escolher_vaga_de_tatica', 'HUMAN_MANAGER::select_tactic_slot', '0x5fbe20', 'human_manager.cpp', 'thiscall, ret 0xc',
          'a vaga ativa do técnico humano: 4 na segunda seleção, 3 com reserva numa seleção, 2 numa seleção, 1 com reserva, 0 no time principal',
          'the human manager active slot: 4 for the second national team, 3 with reserve in a national team, 2 in a national team, 1 with reserve, 0 for the first team'),
         ('tatica_ativa_do_humano', 'HUMAN_MANAGER::get_active_tactic', '0x5fbd00', 'human_manager.cpp', 'thiscall, ret 4',
          'o bloco de tática (0x1ecd bytes) da vaga ativa do registro do técnico humano',
          'the tactic block (0x1ecd bytes) of the active slot of the human manager record')])
CAMPOS = [('Tactics +0x5f1 blocos próprios, +0x5f9 blocos prontos (0x18e3 bytes), +0x5fd conta das prontas, +0x601 tabela tática (linhas de 0x91 bytes por id, +0 índice da tática, palavra com sinal); ponteiros/valores; lê (tatica_do_clube)',
           'Tactics +0x5f1 own blocks, +0x5f9 preset blocks (0x18e3 bytes), +0x5fd preset count, +0x601 tactic table (0x91-byte rows per id, +0 tactic index, signed word); pointers/values; reads (tatica_do_clube)'),
          ('HUMAN_MANAGER +0 registros (0xa6ef bytes, índice id − [0xae2364] + 16), +4 conta; registro +0x9a69 vaga ativa (byte com sinal), blocos de tática de 0x1ecd bytes; valores; lê e grava +0x9a69 (escolher_vaga_de_tatica), lê (tatica_ativa_do_humano)',
           'HUMAN_MANAGER +0 records (0xa6ef bytes, index id − [0xae2364] + 16), +4 count; record +0x9a69 active slot (signed byte), 0x1ecd-byte tactic blocks; values; reads and writes +0x9a69 (escolher_vaga_de_tatica), reads (tatica_ativa_do_humano)'),
          ('clube +0 id, +0xcf técnico; pessoa +0x39 clube; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] humano da vez, [0xb63c98] gerente dos humanos; ponteiros/ids; lê',
           'club +0 id, +0xcf manager; staff +0x39 club; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] current human, [0xb63c98] human manager; pointers/ids; reads')]
INTERESSA = ['CMRTE']
