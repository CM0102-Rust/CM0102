# A copa (Cup.cpp) e a escala pela rodada (comp_util.cpp): o sorteio dos times da rodada (Cup::randomize_round_teams
# 0x51e220, só pela rodada diária da copa 0x51c5f1 quando a rodada não tem potes nem confrontos prontos), os
# confrontos no estádio compartilhado (Cup::separate_shared_stadium_ties 0x522610, logo depois, 0x51c610) e a escala
# pela rodada (comp_util_scale_by_round 0x4bf850: os update_staff_records_for_status das copas; na Inglaterra a
# EngAutoCup 0x56bf13/0x56bf8d, com 5 rodadas). Sem SEH. A 0x51e220 sorteia (0x90cfc0) e usa um vetor temporário
# (malloc/free; sem memória a caixa 0xe7f e o jogo para, como a original); a 0x522610 só grava confrontos; a 0x4bf850
# é pura (fora de 0..=100 a cópia chama a original, que abre a caixa). A 0x4bf850 devolve o eax inteiro (quem chama
# empurra o registrador todo). Iguais no cm0102-1080.exe. Comparadores: o sorteio lado a lado com o estado do
# sorteador (cópia em without_counting) e os confrontos de todas as rodadas; o estádio lado a lado nos confrontos; a escala
# pelo valor.
SAVE = False
ROTINAS = [('copa_sortear_times_da_rodada', 0x51e220, [0x81, 0xec, 0x28, 0x02, 0x00, 0x00]),
           ('copa_separar_estadio_compartilhado', 0x522610, [0x83, 0xec, 0x14, 0x56, 0x57]),
           ('escala_pela_rodada', 0x4bf850, [0x66, 0x8b, 0x4c, 0x24, 0x10])]
DOCS = ('a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada',
        'the cup: the draw of the round teams, the ties at a shared stadium and the scale by round',
        [('copa_sortear_times_da_rodada', 'Cup::randomize_round_teams', '0x51e220', 'Cup.cpp', 'thiscall, ret 4',
          'tira os times dos confrontos da rodada, embaralha e põe cada um num confronto sorteado (primeiro os vazios), sorteando o lado quando as duas vagas estão livres, e acerta o vínculo com a rodada seguinte',
          'takes the teams out of the round ties, shuffles them and puts each one in a drawn tie (empty ones first), drawing the side when both slots are free, and sets the link to the next round'),
         ('copa_separar_estadio_compartilhado', 'Cup::separate_shared_stadium_ties', '0x522610', 'Cup.cpp', 'thiscall, ret 4',
          'inverte o mandante do confronto seguinte cujo time divide o estádio com um time de um confronto anterior da rodada, e acerta o lado nos confrontos ligados',
          'swaps home and away in the later tie whose team shares a stadium with a team of an earlier tie of the round, and sets the side in the linked ties'),
         ('escala_pela_rodada', 'comp_util_scale_by_round', '0x4bf850', 'comp_util.cpp', 'cdecl',
          'o valor da rodada entre dois limites: a média com uma rodada só, senão (alto − baixo) × rodada / (rodadas − 1); fora de 0..100, a caixa de erro',
          'the round value between two limits: the average with a single round, otherwise (high − low) × round / (rounds − 1); outside 0..100, the error box')])
CAMPOS = [('Cup +0xa3: as rodadas (0x68 bytes: +0x18 vagas de times, +0x1a confrontos); lê (as três da copa)',
           'Cup +0xa3: the rounds (0x68 bytes: +0x18 team slots, +0x1a ties); reads (the three cup ones)'),
          ('Cup +0xad: os confrontos por rodada (0x35 bytes: +0/+2 times, −1 sem time, −2 vaga livre; +0xe/+0xa e +0x10/+0xc o confronto ligado na rodada seguinte; +6 confronto de origem, +8 lado, +0x12 rodada de origem); lê e grava (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)',
           'Cup +0xad: the ties per round (0x35 bytes: +0/+2 teams, −1 no team, −2 free slot; +0xe/+0xa and +0x10/+0xc the linked tie in the next round; +6 source tie, +8 side, +0x12 source round); reads and writes (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)'),
          ('Cup +0xa7: os times (6 bytes: +0 clube, +4 nível); lê (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)',
           'Cup +0xa7: the teams (6 bytes: +0 club, +4 level); reads (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)'),
          ('club +0x69: o estádio; ponteiro; lê (copa_separar_estadio_compartilhado: dois clubes com o mesmo estádio não nulo)',
           'club +0x69: the stadium; pointer; reads (copa_separar_estadio_compartilhado: two clubs with the same non-null stadium)')]
INTERESSA = ['SGE', 'GPF2']
