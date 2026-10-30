# A lista de alvos dos clubes (shortlist_manager.cpp): as vagas da formação (shortlist_set_up_formation_slots
# 0x83c090, pela 0x83cb40 que reavalia o elenco de cada clube do computador: o papel das 40 vagas e a máscara de
# posição das 25 vagas de jogador pela formação da tática do clube; a tabela das 10 linhas saiu da emulação do switch),
# refazer a lista do clube (Shortlist_Manager::update_club_shortlist 0x83a730, do laço diário 0x5cccd0: soltar os
# alvos velhos, reavaliar o elenco e redistribuir a verba) e suspender os clubes que miram o jogador
# (Shortlist_Manager::suspend_clubs_shortlisting_player 0x842b00: lesão 0x62d250, regen 0x7a59e0 e transferências;
# estado 100 na entrada do clube e, com limpar, sem a marca de lance nas vagas). Sem sorteio nas três (a 0x83cb40
# chamada pela 0x83a730 sorteia, mas fica na original). Iguais no cm0102-1080.exe. Comparadores: a 0x83c090 lado a
# lado pelas 40 vagas; a 0x83a730 só a original; a 0x842b00 lado a lado pelas entradas e vagas dos clubes do plano
# (a 0x842180 pura chamada antes numa lista própria).
SAVE = False
ROTINAS = [('montar_vagas_da_formacao', 0x83c090, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('refazer_lista_do_clube', 0x83a730, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('suspender_clubes_que_miram_o_jogador', 0x842b00, [0x6a, 0xff, 0x68, 0xdb, 0x24, 0x96, 0x00])]
DOCS = ('a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador',
        'the clubs\' shortlists: the formation slots, rebuilding the club\'s shortlist and suspending the clubs that target the player',
        [('montar_vagas_da_formacao', 'shortlist_set_up_formation_slots', '0x83c090', 'shortlist_manager.cpp', 'stdcall, ret 4',
          'dá o papel das 40 vagas da entrada do clube (25 de jogador, 15 de comissão) e grava a máscara de posição das 25 vagas de jogador pela formação (+7)',
          'sets the role of the 40 slots of the club\'s entry (25 player slots, 15 staff slots) and writes the position mask of the 25 player slots by formation (+7)'),
         ('refazer_lista_do_clube', 'Shortlist_Manager::update_club_shortlist', '0x83a730', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'solta os alvos velhos das vagas do clube, reavalia o elenco (0x83cb40) e redistribui a verba de transferências (0x83b6e0)',
          'releases the stale targets of the club\'s slots, reassesses the squad (0x83cb40) and shares out the transfer budget again (0x83b6e0)'),
         ('suspender_clubes_que_miram_o_jogador', 'Shortlist_Manager::suspend_clubs_shortlisting_player', '0x842b00', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'põe em 100 o estado da entrada de cada clube que mira o jogador (0x842180) e, com limpar, tira a marca de lance das vagas com ele',
          'sets to 100 the entry state of every club that targets the player (0x842180) and, with clear, removes the bid marker from the slots holding him')])
CAMPOS = [('entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; id × 9) +0: as 40 vagas de 0x25 bytes; ponteiro; lê (as três)',
           'club\'s 9-byte entry in Shortlist_Manager (+0xdc; id × 9) +0: the 40 slots of 0x25 bytes; pointer; reads (all three)'),
          ('entrada do clube +7: a formação (1..9; fora, a padrão); valor; lê (montar_vagas_da_formacao)',
           'club entry +7: the formation (1..9; otherwise the default); value; reads (montar_vagas_da_formacao)'),
          ('entrada do clube +8: o estado (100 = suspenso para o jogador); valor; grava 100 (suspender_clubes_que_miram_o_jogador)',
           'club entry +8: the state (100 = suspended for the player); value; writes 100 (suspender_clubes_que_miram_o_jogador)'),
          ('vaga de 0x25 bytes +0: o papel (0xb jogador; 6, 7, 8, 10, 9 comissão); valor; grava (montar_vagas_da_formacao)',
           '0x25-byte slot +0: the role (0xb player; 6, 7, 8, 10, 9 staff); value; writes (montar_vagas_da_formacao)'),
          ('vaga +1: a máscara de posição (16 bits); valor; grava (montar_vagas_da_formacao)',
           'slot +1: the position mask (16 bits); value; writes (montar_vagas_da_formacao)'),
          ('vaga +0x24: bits 2..4 = 0 e bit 1 nas vagas de comissão; valor; lê e grava (montar_vagas_da_formacao)',
           'slot +0x24: bits 2..4 = 0 and bit 1 in the staff slots; value; reads and writes (montar_vagas_da_formacao)'),
          ('vaga +0xf: o id do jogador alvo; id; lê (suspender_clubes_que_miram_o_jogador)',
           'slot +0xf: the target player id; id; reads (suspender_clubes_que_miram_o_jogador)'),
          ('vaga +0x17: a marca de lance (−1 sem); valor; lê e grava −1 (suspender_clubes_que_miram_o_jogador)',
           'slot +0x17: the bid marker (−1 none); value; reads and writes −1 (suspender_clubes_que_miram_o_jogador)')]
INTERESSA = ['CMRTE', 'CMScout']
