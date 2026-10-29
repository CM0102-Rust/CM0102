# O jogador que se aposenta e o clube do jovem (player_regen.cpp): tirar o aposentado dos elencos do clube e da seleção
# e trocar o cargo (PLAYER_REGEN::retire_from_playing_staff 0x7aace0, 248 chamadas na carreira até 2002-06-01), o
# clube de maior nota para o jovem entre todos (PLAYER_REGEN::pick_best_club_for_player 0x7ad0e0, 82) e o clube
# sorteado da lista da nação dele (PLAYER_REGEN::pick_club_from_nation_list 0x7ad590, 111). Sem SEH; a 0x7ad0e0 e a
# 0x7ad590 sorteiam (a 0x7ad220 e a nota), a 0x7aace0 não. Chamam a 0x8ce990 (TRANSFER_MANAGER::can_club_bid_for_player,
# ainda em C, pelo endereço) e a 0x896c60 (Tactics, idem). Iguais no cm0102-1080.exe. Comparadores: lado a lado (a
# aposentadoria pelas vagas e pela pessoa; as escolhas a partir do mesmo estado do sorteio, pelo clube devolvido).
SAVE = False
ROTINAS = [('aposentar_do_elenco', 0x7aace0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('melhor_clube_para_o_jovem', 0x7ad0e0, [0x83, 0xec, 0x14, 0x89, 0x4c, 0x24, 0x10]),
           ('clube_da_nacao_para_o_jovem', 0x7ad590, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('o jogador que se aposenta sai dos elencos e a escolha do clube do jovem: o de maior nota entre todos e o sorteado da lista da nação',
        'the retiring player leaves the squads and the choice of the youngster\'s club: the best scored among all and the drawn one from his nation\'s list',
        [('aposentar_do_elenco', 'PLAYER_REGEN::retire_from_playing_staff', '0x7aace0', 'player_regen.cpp', 'stdcall, ret 4',
          'tira o jogador da tática e das 50 vagas do clube (+0x39) e da seleção (+0x24) e troca o cargo de jogador pelo da comissão (12 → 5, 13 → 6, 15 → 8, os outros 0x10 aposentado); sem cargo que fica, zera o clube e a seleção',
          'removes the player from the tactics and the 50 slots of his club (+0x39) and national team (+0x24) and changes the player job to the staff job (12 → 5, 13 → 6, 15 → 8, the others 0x10 retired); without a kept job, clears the club and the national team'),
         ('melhor_clube_para_o_jovem', 'PLAYER_REGEN::pick_best_club_for_player', '0x7ad0e0', 'player_regen.cpp', 'thiscall, ret 4',
          'entre os clubes em que o jovem não é estrangeiro, que não são time B, que podem chegar a ele e com elenco abaixo de 50, o de maior nota (0x7ad220)',
          'among the clubs where the youngster is not a foreigner, that are not a B team, that may approach him and with a squad under 50, the best scored one (0x7ad220)'),
         ('clube_da_nacao_para_o_jovem', 'PLAYER_REGEN::pick_club_from_nation_list', '0x7ad590', 'player_regen.cpp', 'thiscall, ret 4',
          'na lista de clubes da nação do jovem (algumas nações trocadas por [0x9cf2e4]), o de maior sorteio da nota pela reputação contra a habilidade, entre os que podem chegar a ele e com elenco abaixo de 50',
          'in the club list of the youngster\'s nation (some nations mapped to [0x9cf2e4]), the one with the highest draw of the score by reputation against ability, among those that may approach him and with a squad under 50')])
CAMPOS = [('staff +0x39 / +0x24: o clube e a seleção da pessoa; ponteiro; lê e grava 0 (aposentar_do_elenco)',
           'staff +0x39 / +0x24: the person\'s club and national team; pointer; reads and writes 0 (aposentar_do_elenco)'),
          ('staff +0x3d / +0x28: o cargo no clube e na seleção (11 jogador, 12/13/15 jogador e comissão, 16 aposentado; 5, 6, 8 comissão); valor; lê e grava (aposentar_do_elenco)',
           'staff +0x3d / +0x28: the job at the club and at the national team (11 player, 12/13/15 player and staff, 16 retired; 5, 6, 8 staff); value; reads and writes (aposentar_do_elenco)'),
          ('clube +0xd7: as 50 vagas do elenco; ponteiro; lê e grava 0 (aposentar_do_elenco)',
           'club +0xd7: the 50 squad slots; pointer; reads and writes 0 (aposentar_do_elenco)'),
          ('staff +0x1a / +0x1e: a nação e a segunda nação; ponteiro; lê (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)',
           'staff +0x1a / +0x1e: nation and second nation; pointer; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)'),
          ('staff +0x61 → ficha +5, +7, +0xb: a habilidade atual, a potencial e a reputação; valor; lê (clube_da_nacao_para_o_jovem)',
           'staff +0x61 → player data +5, +7, +0xb: current ability, potential and reputation; value; reads (clube_da_nacao_para_o_jovem)'),
          ('clube +0x53 / +0x80: a nação e a reputação; ponteiro e valor; lê (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)',
           'club +0x53 / +0x80: nation and reputation; pointer and value; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)'),
          ('PLAYER_REGEN +0 → [4 × nação]: a lista de clubes da nação (+0 conta, +2 cabeça, +0xa cursor); ponteiro; lê e grava o cursor (clube_da_nacao_para_o_jovem)',
           'PLAYER_REGEN +0 → [4 × nation]: the nation\'s club list (+0 count, +2 head, +0xa cursor); pointer; reads and writes the cursor (clube_da_nacao_para_o_jovem)'),
          ('[0x9cf424], [0x9d0120], [0x9d0124], [0x9d0128], [0x9d0134], [0x9d01a0]: a nação e os clubes do veto; id; lê (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)',
           '[0x9cf424], [0x9d0120], [0x9d0124], [0x9d0128], [0x9d0134], [0x9d01a0]: the veto nation and clubs; id; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)'),
          ('[0x9cf510], [0x9cf2dc], [0x9cf3e4] → [0x9cf2e4]: as nações cuja lista é a de outra; id; lê (clube_da_nacao_para_o_jovem)',
           '[0x9cf510], [0x9cf2dc], [0x9cf3e4] → [0x9cf2e4]: the nations whose list is another one\'s; id; reads (clube_da_nacao_para_o_jovem)')]
INTERESSA = ['GPF2', 'CMRTE']
